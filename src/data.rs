//! All game data the bot uses, loaded once and swapped atomically by `/reload`
//!
//! Commands take a `&GameData` snapshot, so a reload never changes data in the middle of a
//! command.

use std::collections::{BTreeMap, HashMap};
use std::path::Path;
use std::sync::Arc;
use std::time::Instant;

use arc_swap::ArcSwapOption;
use tracing::{info, warn};

use crate::cdclient::components::{
    DESTROYABLE_COMPONENT, ITEM_COMPONENT, MISSION_OFFER_COMPONENT, PACKAGE_COMPONENT,
};
use crate::cdclient::{CdClient, TableWarning};
use crate::config::BotConfig;
use crate::error::{BotError, BotResult, DataLoadError};
use crate::locale::{load_locale, LocaleTranslation};
use crate::search::{SearchCategory, SearchEntry, SearchIndex};

/// Fan-made loot table names (`LootTableIndex<TAB>name`)
const LOOT_TABLE_NAMES: &str = include_str!("../data/loot_table_names.tsv");

pub struct GameData {
    pub cdclient: CdClient,
    pub locale: LocaleTranslation,
    pub loot_table_names: HashMap<i32, String>,
    pub index: Indexes,
    pub search: SearchIndex,
    pub hq_valid_only: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct NpcMission {
    pub mission: i32,
    pub offers: bool,
    pub accepts: bool,
}

/// Reverse lookups that would otherwise need full table scans
#[derive(Default)]
pub struct Indexes {
    /// `(component type, component id)` -> objects with that component
    pub objects_with_component: HashMap<(i32, i32), Vec<i32>>,
    /// item -> loot tables containing it
    pub loot_tables_with_item: HashMap<i32, Vec<i32>>,
    /// loot table -> loot matrices using it
    pub loot_matrices_with_table: HashMap<i32, Vec<i32>>,
    /// loot matrix -> destructible component ids
    pub destructibles_with_matrix: HashMap<i32, Vec<i32>>,
    /// loot matrix -> package component ids
    pub packages_with_matrix: HashMap<i32, Vec<i32>>,
    /// loot matrix -> vendor component ids
    pub vendors_with_matrix: HashMap<i32, Vec<i32>>,
    /// loot matrix -> activity ids (`ActivityRewards.objectTemplate`)
    pub activities_with_matrix: HashMap<i32, Vec<i32>>,
    /// item -> missions that reward it
    pub missions_rewarding_item: HashMap<i32, Vec<i32>>,
    /// object -> missions it offers/accepts
    pub npc_missions: HashMap<i32, Vec<NpcMission>>,
    pub skills_in_cooldown_group: BTreeMap<i32, Vec<i32>>,
    pub objects_with_skill: HashMap<i32, Vec<i32>>,
    /// item -> item sets (kits) that contain it
    pub item_sets_with_item: HashMap<i32, Vec<i32>>,
    /// item -> rarity (from its item component)
    pub item_rarity: HashMap<i32, i32>,
    /// `(loot table, rarity)` -> number of items of that rarity; `None` counts every item
    pub loot_table_rarity_counts: HashMap<(i32, Option<i32>), usize>,
}

fn push_unique<K: std::hash::Hash + Eq>(map: &mut HashMap<K, Vec<i32>>, key: K, value: i32) {
    let values = map.entry(key).or_default();
    if !values.contains(&value) {
        values.push(value);
    }
}

fn sort_values<K>(map: &mut HashMap<K, Vec<i32>>) {
    for values in map.values_mut() {
        values.sort_unstable();
    }
}

impl Indexes {
    fn build(cd: &CdClient) -> Self {
        let mut ix = Indexes::default();

        for cr in cd.components_registry.iter() {
            push_unique(&mut ix.objects_with_component, (cr.component_type, cr.component_id), cr.id);
        }

        for lt in cd.loot_table.iter() {
            push_unique(&mut ix.loot_tables_with_item, lt.itemid, lt.loot_table_index);
        }
        for lm in cd.loot_matrix.iter() {
            push_unique(&mut ix.loot_matrices_with_table, lm.loot_table_index, lm.loot_matrix_index);
        }
        for dc in cd.destructible_component.iter() {
            if let Some(lmi) = dc.loot_matrix_index {
                push_unique(&mut ix.destructibles_with_matrix, lmi, dc.id);
            }
        }
        for pc in cd.package_component.iter() {
            push_unique(&mut ix.packages_with_matrix, pc.loot_matrix_index, pc.id);
        }
        for vc in cd.vendor_component.iter() {
            push_unique(&mut ix.vendors_with_matrix, vc.loot_matrix_index, vc.id);
        }
        for ar in cd.activity_rewards.iter() {
            if let Some(lmi) = ar.loot_matrix_index {
                push_unique(&mut ix.activities_with_matrix, lmi, ar.object_template);
            }
        }

        for mission in cd.missions.iter() {
            for item in [
                mission.reward_item1,
                mission.reward_item2,
                mission.reward_item3,
                mission.reward_item4,
            ] {
                if item > 0 {
                    push_unique(&mut ix.missions_rewarding_item, item, mission.id);
                }
            }
        }

        // NPCs offer/accept missions through their mission offer component
        let mut npc_missions: HashMap<i32, Vec<NpcMission>> = HashMap::new();
        for row in cd.mission_npccomponent.iter() {
            let Some(objects) = ix.objects_with_component.get(&(MISSION_OFFER_COMPONENT, row.id)) else {
                continue;
            };
            for &object in objects {
                let entries = npc_missions.entry(object).or_default();
                match entries.iter_mut().find(|e| e.mission == row.mission_id) {
                    Some(entry) => {
                        entry.offers |= row.offers_mission;
                        entry.accepts |= row.accepts_mission;
                    }
                    None => entries.push(NpcMission {
                        mission: row.mission_id,
                        offers: row.offers_mission,
                        accepts: row.accepts_mission,
                    }),
                }
            }
        }
        // some missions only list their giver/receiver on the mission itself
        for mission in cd.missions.iter() {
            for (object, offers) in [
                (mission.offer_object_id, true),
                (mission.target_object_id, false),
            ] {
                let Some(object) = object.filter(|&o| o > 0) else {
                    continue;
                };
                let entries = npc_missions.entry(object).or_default();
                match entries.iter_mut().find(|e| e.mission == mission.id) {
                    Some(entry) if offers => entry.offers = true,
                    Some(entry) => entry.accepts = true,
                    None => entries.push(NpcMission {
                        mission: mission.id,
                        offers,
                        accepts: !offers,
                    }),
                }
            }
        }
        for entries in npc_missions.values_mut() {
            entries.sort_by_key(|e| e.mission);
        }
        ix.npc_missions = npc_missions;

        for skill in cd.skill_behavior.iter() {
            if let Some(group) = skill.cooldowngroup {
                ix.skills_in_cooldown_group.entry(group).or_default().push(skill.skill_id);
            }
        }
        for os in cd.object_skills.iter() {
            push_unique(&mut ix.objects_with_skill, os.skill_id, os.object_template);
        }
        for set in cd.item_sets.iter() {
            for &item in &set.item_ids {
                push_unique(&mut ix.item_sets_with_item, item, set.set_id);
            }
        }

        for cr in cd.components_registry.iter() {
            if cr.component_type != ITEM_COMPONENT {
                continue;
            }
            if let Some(rarity) = cd.item_component.at_key(&cr.component_id).and_then(|ic| ic.rarity) {
                ix.item_rarity.insert(cr.id, rarity);
            }
        }
        for lt in cd.loot_table.iter() {
            let rarity = ix.item_rarity.get(&lt.itemid).copied();
            *ix.loot_table_rarity_counts.entry((lt.loot_table_index, None)).or_default() += 1;
            if rarity.is_some() {
                *ix.loot_table_rarity_counts.entry((lt.loot_table_index, rarity)).or_default() += 1;
            }
        }

        for map in [
            &mut ix.loot_tables_with_item,
            &mut ix.loot_matrices_with_table,
            &mut ix.destructibles_with_matrix,
            &mut ix.packages_with_matrix,
            &mut ix.vendors_with_matrix,
            &mut ix.activities_with_matrix,
            &mut ix.missions_rewarding_item,
            &mut ix.objects_with_skill,
            &mut ix.item_sets_with_item,
        ] {
            sort_values(map);
        }
        sort_values(&mut ix.objects_with_component);
        ix
    }
}

fn parse_loot_table_names(text: &str) -> HashMap<i32, String> {
    text.lines()
        .filter(|line| !line.starts_with('#'))
        .filter_map(|line| {
            let (id, name) = line.split_once('\t')?;
            Some((id.trim().parse().ok()?, name.trim().to_string()))
        })
        .collect()
}

impl GameData {
    /// Loads the cdclient and locale and builds all indexes; this is slow (a few seconds), so
    /// call it from a blocking context
    pub fn load(config: &BotConfig) -> Result<(Self, Vec<TableWarning>), DataLoadError> {
        let start = Instant::now();
        let (cdclient, warnings) = CdClient::load_sqlite(&config.cdclient_path)?;
        info!("loaded {} in {:?}", config.cdclient_path.display(), start.elapsed());

        let start = Instant::now();
        let locale = load_locale(&config.locale_path, &config.locale)?;
        info!("loaded {} ({}) in {:?}", config.locale_path.display(), config.locale, start.elapsed());

        for warning in &warnings {
            warn!("table `{}` could not be loaded and will be empty: {}", warning.table, warning.error);
        }

        Ok((Self::from_parts(cdclient, locale, config.hq_valid_only), warnings))
    }

    pub fn from_parts(cdclient: CdClient, locale: LocaleTranslation, hq_valid_only: bool) -> Self {
        let start = Instant::now();
        let index = Indexes::build(&cdclient);
        let mut data = GameData {
            cdclient,
            locale,
            loot_table_names: parse_loot_table_names(LOOT_TABLE_NAMES),
            index,
            search: SearchIndex::default(),
            hq_valid_only,
        };
        data.search = build_search_index(&data);
        info!("built indexes in {:?}", start.elapsed());
        data
    }

    /// Whether an object should be shown, honoring `HQ_VALID_ONLY`
    pub fn is_visible(&self, object: i32) -> bool {
        !self.hq_valid_only
            || self
                .cdclient
                .objects
                .at_key(&object)
                .is_some_and(|o| o.hq_valid.unwrap_or(false))
    }

    pub fn objects_with_component(&self, component_type: i32, component_id: i32) -> &[i32] {
        self.index
            .objects_with_component
            .get(&(component_type, component_id))
            .map_or(&[], Vec::as_slice)
    }
}

fn build_search_index(d: &GameData) -> SearchIndex {
    use crate::ids::{CdClientActivityId, CdClientLootTableId, CdClientMissionsId, CdClientObjectsId, CdClientSkillBehaviorId, LUExplorer};

    let cd = &d.cdclient;
    let mut search = SearchIndex::default();

    // ------- //
    // Objects //
    // ------- //
    let has_component = |object: i32, component_type: i32| {
        cd.components_registry
            .at_group_key(&object)
            .is_some_and(|crs| crs.iter().any(|cr| cr.component_type == component_type))
    };
    let mut objects = vec![];
    let mut by_category: HashMap<SearchCategory, Vec<SearchEntry>> = HashMap::new();
    for object in cd.objects.iter() {
        if !d.is_visible(object.id) {
            continue;
        }
        let id = CdClientObjectsId(object.id);
        let localized = d.locale.objects.get(&object.id).and_then(|o| o.name.as_ref());
        let mut entry = SearchEntry::new(object.id, id.req_name(d)).bonus(if localized.is_some() { 100 } else { 0 });
        for alias in [&object.display_name, &object.name].into_iter().flatten() {
            entry = entry.alias(alias);
        }
        if let Some(kind) = &object.r#type {
            entry = entry.detail(kind.clone());
        }

        let kind = object.r#type.as_deref().unwrap_or_default();
        if has_component(object.id, ITEM_COMPONENT) {
            by_category.entry(SearchCategory::Item).or_default().push(entry.clone());
        }
        if kind == "LEGO brick" {
            by_category.entry(SearchCategory::Brick).or_default().push(entry.clone());
        }
        if kind == "Enemies" && has_component(object.id, DESTROYABLE_COMPONENT) {
            by_category.entry(SearchCategory::Enemy).or_default().push(entry.clone());
        }
        let drops_loot = id
            .component_ids(d, DESTROYABLE_COMPONENT)
            .iter()
            .any(|&comp| cd.destructible_component.at_key(&comp).is_some_and(|dc| dc.loot_matrix_index.is_some()));
        if drops_loot {
            by_category.entry(SearchCategory::Smashable).or_default().push(entry.clone());
        }
        if has_component(object.id, PACKAGE_COMPONENT) {
            by_category.entry(SearchCategory::Package).or_default().push(entry.clone());
        }
        if id.vendor_loot_matrix(d).is_some() {
            by_category.entry(SearchCategory::Vendor).or_default().push(entry.clone());
        }
        if d.index.npc_missions.contains_key(&object.id) {
            by_category.entry(SearchCategory::Npc).or_default().push(entry.clone());
        }
        objects.push(entry);
    }
    search.insert(SearchCategory::Object, objects);
    for (category, entries) in by_category {
        search.insert(category, entries);
    }

    // -------- //
    // Missions //
    // -------- //
    let mut missions = vec![];
    let mut achievements = vec![];
    for mission in cd.missions.iter() {
        let id = CdClientMissionsId(mission.id);
        let named = id.name(d).is_some();
        let mut entry = SearchEntry::new(mission.id, id.req_name(d))
            .alias(&mission.defined_type)
            .detail(id.category_path(d))
            .bonus(if named { 100 } else { 0 });
        if let Some(subtype) = &mission.defined_subtype {
            entry = entry.alias(subtype);
        }
        if mission.is_mission {
            missions.push(entry);
        } else {
            achievements.push(entry);
        }
    }
    search.insert(SearchCategory::Mission, missions);
    search.insert(SearchCategory::Achievement, achievements);

    // ------ //
    // Skills //
    // ------ //
    let skills = cd
        .skill_behavior
        .iter()
        .map(|skill| {
            let id = CdClientSkillBehaviorId(skill.skill_id);
            SearchEntry::new(skill.skill_id, id.req_name(d)).bonus(if id.name(d).is_some() { 100 } else { 0 })
        })
        .collect();
    search.insert(SearchCategory::Skill, skills);

    let groups = d
        .index
        .skills_in_cooldown_group
        .iter()
        .map(|(&group, skills)| {
            let mut entry = SearchEntry::new(group, format!("Cooldown Group {group}"));
            for &skill in skills {
                if let Some(name) = CdClientSkillBehaviorId(skill).name(d) {
                    entry = entry.alias(name);
                }
            }
            entry
        })
        .collect();
    search.insert(SearchCategory::CooldownGroup, groups);

    // ---------- //
    // Activities //
    // ---------- //
    let mut activity_ids: Vec<i32> = cd.activity_rewards.iter().map(|ar| ar.object_template).collect();
    activity_ids.dedup();
    let activities = activity_ids
        .into_iter()
        .map(|activity| {
            let id = CdClientActivityId(activity);
            let mut entry = SearchEntry::new(activity, id.req_name(d)).bonus(100);
            for reward in cd.activity_rewards.at_group_key(&activity).unwrap_or_default() {
                entry = entry.alias(&reward.description);
            }
            entry
        })
        .collect();
    search.insert(SearchCategory::Activity, activities);

    // ----------- //
    // Loot Tables //
    // ----------- //
    let mut ltis: Vec<i32> = cd.loot_table.iter().map(|lt| lt.loot_table_index).collect();
    ltis.dedup();
    let loot_tables = ltis
        .into_iter()
        .map(|lti| {
            let named = d.loot_table_names.contains_key(&lti);
            SearchEntry::new(lti, CdClientLootTableId(lti).req_name(d)).bonus(if named { 100 } else { 0 })
        })
        .collect();
    search.insert(SearchCategory::LootTable, loot_tables);

    search
}

// ------------ //
// Global State //
// ------------ //

static DATA: ArcSwapOption<GameData> = ArcSwapOption::const_empty();

/// The current game data
pub fn get() -> BotResult<Arc<GameData>> {
    DATA.load_full()
        .ok_or_else(|| BotError::internal("game data has not been loaded"))
}

pub fn install(data: GameData) {
    DATA.store(Some(Arc::new(data)));
}

/// Loads fresh data and swaps it in; the old data stays live if anything fails
pub fn reload(config: &BotConfig) -> Result<Vec<TableWarning>, DataLoadError> {
    let (data, warnings) = GameData::load(config)?;
    install(data);
    Ok(warnings)
}

/// Loads data from explicit paths (used by tests and the `check` subcommand)
pub fn load_from_paths(cdclient: &Path, locale: &Path, locale_name: &str) -> Result<GameData, DataLoadError> {
    let (cd, warnings) = CdClient::load_sqlite(cdclient)?;
    for warning in &warnings {
        warn!("table `{}` could not be loaded and will be empty: {}", warning.table, warning.error);
    }
    let locale = load_locale(locale, locale_name)?;
    Ok(GameData::from_parts(cd, locale, false))
}
