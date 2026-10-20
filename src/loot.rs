//! Drop chance math
//!
//! A loot matrix is a list of rolls. For each roll (a `LootMatrix` row):
//! 1. the roll happens with probability `percent`
//! 2. `minToDrop..=maxToDrop` items are dropped (uniformly chosen)
//! 3. for each drop, a rarity is chosen from the rarity table (`randmax` is cumulative)
//! 4. an item of that rarity is chosen uniformly from the loot table
//!
//! Items without a rarity (e.g. power-ups) are chosen uniformly from the whole table.

use std::collections::BTreeMap;

use crate::cdclient::components::{DESTROYABLE_COMPONENT, PACKAGE_COMPONENT};
use crate::cdclient::CdClientLootMatrix;
use crate::data::GameData;
use crate::ids::{CdClientLootTableId, CdClientObjectsId};

/// Chance of rolling exactly `rarity` from a rarity table
pub fn rarity_chance(d: &GameData, rarity_table: i32, rarity: i32) -> f64 {
    let Some(rows) = d.cdclient.rarity_table.at_group_key(&rarity_table) else {
        return 0.0;
    };
    let Some(upper) = rows.iter().filter(|r| r.rarity == rarity).map(|r| r.randmax).reduce(f64::max) else {
        return 0.0;
    };
    // randmax is cumulative, so subtract the highest bound below this rarity (rarities can be
    // skipped, so this is not always `rarity - 1`)
    let lower = rows
        .iter()
        .filter(|r| r.rarity < rarity)
        .map(|r| r.randmax)
        .fold(0.0, f64::max);
    (upper - lower).clamp(0.0, 1.0)
}

/// Every rarity in a rarity table with its chance, lowest rarity first
pub fn rarity_distribution(d: &GameData, rarity_table: i32) -> Vec<(i32, f64)> {
    let mut rarities: Vec<i32> = d
        .cdclient
        .rarity_table
        .at_group_key(&rarity_table)
        .unwrap_or_default()
        .iter()
        .map(|r| r.rarity)
        .collect();
    rarities.sort_unstable();
    rarities.dedup();
    rarities
        .into_iter()
        .map(|rarity| (rarity, rarity_chance(d, rarity_table, rarity)))
        .collect()
}

/// Chance that one dropped item from this row is `item`
fn chance_per_drop(d: &GameData, item: CdClientObjectsId, row: &CdClientLootMatrix) -> f64 {
    let table = CdClientLootTableId(row.loot_table_index);
    match item.rarity(d) {
        Some(rarity) => {
            let count = table.count(d, Some(rarity));
            if count == 0 {
                return 0.0;
            }
            rarity_chance(d, row.rarity_table_index, rarity) / count as f64
        }
        None => {
            let count = table.count(d, None);
            if count == 0 {
                return 0.0;
            }
            1.0 / count as f64
        }
    }
}

/// Chance that a single loot matrix row gives at least one of the item
fn row_chance(d: &GameData, item: CdClientObjectsId, row: &CdClientLootMatrix) -> f64 {
    let per_drop = chance_per_drop(d, item, row);
    if per_drop <= 0.0 {
        return 0.0;
    }
    let min = row.min_to_drop.max(0);
    let max = row.max_to_drop.max(min);
    let count = (max - min + 1) as f64;
    // average over how many items are dropped
    let at_least_one: f64 = (min..=max)
        .map(|k| 1.0 - (1.0 - per_drop).powi(k))
        .sum::<f64>()
        / count;
    (row.percent.clamp(0.0, 1.0) * at_least_one).clamp(0.0, 1.0)
}

/// Chance that one use of a loot matrix (one smash, one package, …) gives at least one `item`
pub fn item_chance(d: &GameData, item: CdClientObjectsId, loot_matrix: i32) -> f64 {
    let tables = item.loot_tables(d);
    let miss: f64 = d
        .cdclient
        .loot_matrix
        .at_group_key(&loot_matrix)
        .unwrap_or_default()
        .iter()
        .filter(|row| tables.contains(&row.loot_table_index))
        .map(|row| 1.0 - row_chance(d, item, row))
        .product();
    1.0 - miss
}

/// Where an item can come from
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LootSource {
    /// smashing objects (destructible component)
    Smashable,
    /// opening packages
    Package,
    /// activity rewards
    Activity,
}

/// Something that gives an item, and the loot matrix it uses
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Source {
    /// object id, or activity id for [`LootSource::Activity`]
    pub id: i32,
    pub loot_matrix: i32,
}

/// Sources with the same chance
#[derive(Clone, Debug)]
pub struct SourceGroup {
    pub chance: f64,
    pub sources: Vec<Source>,
}

/// Every source of an item, grouped by loot matrix, highest chance first
pub fn item_sources(d: &GameData, item: CdClientObjectsId, kind: LootSource) -> Vec<SourceGroup> {
    let mut groups = vec![];
    for lmi in item.loot_matrices(d) {
        let ids: Vec<i32> = match kind {
            LootSource::Smashable => objects_using(d, d.index.destructibles_with_matrix.get(&lmi), DESTROYABLE_COMPONENT),
            LootSource::Package => objects_using(d, d.index.packages_with_matrix.get(&lmi), PACKAGE_COMPONENT),
            LootSource::Activity => d.index.activities_with_matrix.get(&lmi).cloned().unwrap_or_default(),
        };
        if ids.is_empty() {
            continue;
        }
        let sources: Vec<Source> = ids.into_iter().map(|id| Source { id, loot_matrix: lmi }).collect();
        let chance = item_chance(d, item, lmi);
        if chance <= 0.0 {
            continue;
        }
        // sources with the same chance read best as one list
        match groups.iter_mut().find(|g: &&mut SourceGroup| same_chance(g.chance, chance)) {
            Some(group) => {
                for source in sources {
                    if !group.sources.contains(&source) {
                        group.sources.push(source);
                    }
                }
            }
            None => groups.push(SourceGroup { chance, sources }),
        }
    }
    sort_groups(&mut groups);
    groups
}

fn same_chance(a: f64, b: f64) -> bool {
    (a - b).abs() <= 1e-9 * a.abs().max(b.abs()).max(1e-12)
}

fn objects_using(d: &GameData, components: Option<&Vec<i32>>, component_type: i32) -> Vec<i32> {
    let mut objects: Vec<i32> = components
        .into_iter()
        .flatten()
        .flat_map(|&comp| d.objects_with_component(component_type, comp).iter().copied())
        .filter(|&object| d.is_visible(object))
        .collect();
    objects.sort_unstable();
    objects.dedup();

    // objects with no in-game name are almost always unused test/dev objects; hide them unless
    // they are the only sources
    let named: Vec<i32> = objects
        .iter()
        .copied()
        .filter(|id| d.locale.objects.get(id).is_some_and(|o| o.name.is_some()))
        .collect();
    if named.is_empty() {
        objects
    } else {
        named
    }
}

fn sort_groups(groups: &mut [SourceGroup]) {
    groups.sort_by(|a, b| {
        b.chance
            .total_cmp(&a.chance)
            .then(a.sources.first().map(|s| s.id).cmp(&b.sources.first().map(|s| s.id)))
    });
}

/// One roll of a loot matrix, described for display
#[derive(Clone, Debug)]
pub struct LootRoll {
    pub loot_table: CdClientLootTableId,
    pub percent: f64,
    pub min: i32,
    pub max: i32,
    /// `(rarity, chance of that rarity, items of that rarity in the table)`
    pub tiers: Vec<(i32, f64, usize)>,
    /// Items in the table without a rarity
    pub unrated: usize,
}

/// Every roll in a loot matrix
pub fn loot_rolls(d: &GameData, loot_matrix: i32) -> Vec<LootRoll> {
    d.cdclient
        .loot_matrix
        .at_group_key(&loot_matrix)
        .unwrap_or_default()
        .iter()
        .map(|row| {
            let table = CdClientLootTableId(row.loot_table_index);
            let tiers: Vec<_> = rarity_distribution(d, row.rarity_table_index)
                .into_iter()
                .map(|(rarity, chance)| (rarity, chance, table.count(d, Some(rarity))))
                .filter(|&(_, chance, count)| chance > 0.0 && count > 0)
                .collect();
            let rated: usize = tiers.iter().map(|t| t.2).sum::<usize>();
            let total = table.count(d, None);
            LootRoll {
                loot_table: table,
                percent: row.percent.clamp(0.0, 1.0),
                min: row.min_to_drop.max(0),
                max: row.max_to_drop.max(row.min_to_drop.max(0)),
                tiers,
                unrated: total.saturating_sub(rated),
            }
        })
        .collect()
}

/// Items in a loot table grouped by rarity (`None` = no rarity)
pub fn items_by_rarity(d: &GameData, table: CdClientLootTableId) -> BTreeMap<Option<i32>, Vec<CdClientObjectsId>> {
    let mut map: BTreeMap<Option<i32>, Vec<CdClientObjectsId>> = BTreeMap::new();
    for item in table.items(d) {
        map.entry(item.rarity(d)).or_default().push(item);
    }
    map
}

/// `0.25` -> `"25.0000% (1 in 4)"`, using the configured decimal places
/// - "1 in N" is only shown below 50%, where it is easier to read than the percentage
pub fn format_chance(chance: f64) -> String {
    let config = crate::config::config();
    if chance >= 1.0 {
        return format!("{} (guaranteed)", config.percent(1.0));
    }
    if chance <= 0.0 {
        return config.percent(0.0);
    }
    let smallest = 10f64.powi(-(config.decimal_places as i32)) / 100.0;
    let percent = if chance < smallest {
        format!("<{}", config.percent(smallest))
    } else {
        config.percent(chance)
    };
    if chance >= 0.5 {
        return percent;
    }
    let one_in = 1.0 / chance;
    let one_in = if one_in < 10.0 {
        format!("{:.1}", one_in).trim_end_matches(".0").to_string()
    } else {
        format_number(one_in.round() as i64)
    };
    format!("{percent} (1 in {one_in})")
}

/// `1234567` -> `"1,234,567"`
pub fn format_number(n: i64) -> String {
    let digits = n.unsigned_abs().to_string();
    let mut out = String::new();
    for (i, ch) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i) % 3 == 0 {
            out.push(',');
        }
        out.push(ch);
    }
    if n < 0 {
        format!("-{out}")
    } else {
        out
    }
}

/// Formats a float stored as f32 in the cdclient without noise: `0.4000000059604645` -> `"0.4"`
pub fn format_float(value: f64) -> String {
    let text = format!("{value:.3}");
    let text = text.trim_end_matches('0').trim_end_matches('.');
    if text == "-0" {
        String::from("0")
    } else {
        text.to_string()
    }
}

/// `"1"` or `"1–3"`
pub fn format_range(min: i32, max: i32) -> String {
    if min == max {
        min.to_string()
    } else {
        format!("{min}–{max}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers() {
        assert_eq!(format_number(0), "0");
        assert_eq!(format_number(999), "999");
        assert_eq!(format_number(11250), "11,250");
        assert_eq!(format_number(-1234567), "-1,234,567");
        assert_eq!(format_float(0.4000000059604645), "0.4");
        assert_eq!(format_float(1.0), "1");
        assert_eq!(format_float(12.5), "12.5");
        assert_eq!(format_range(1, 1), "1");
        assert_eq!(format_range(1, 3), "1–3");
        assert_eq!(format_chance(1.0), "100.0000% (guaranteed)");
        assert_eq!(format_chance(0.75), "75.0000%");
        assert_eq!(format_chance(0.25), "25.0000% (1 in 4)");
        assert_eq!(format_chance(0.1359), "13.5900% (1 in 7.4)");
        assert_eq!(format_chance(0.00007), "0.0070% (1 in 14,286)");
        assert_eq!(format_chance(0.0), "0.0000%");
    }
}
