use crate::data::GameData;
use crate::ids::{CdClientObjectsId, LUExplorer};
use crate::interaction_command::{CommandResult, InteractionCommand, OptionKind, Response, Target};
use crate::loot::format_number;
use crate::search::SearchCategory;

use super::{item_rows, object_embed};

pub struct ItemCommand;

impl InteractionCommand for ItemCommand {
    const NAME: &'static str = "item";
    const DESCRIPTION: &'static str = "View the stats of an item!";
    const OPTION_NAME: &'static str = "item";
    const OPTION_DESCRIPTION: &'static str = "An item in LEGO Universe (name or id).";
    const OPTION: OptionKind = OptionKind::Search(SearchCategory::Item);

    fn run(d: &GameData, target: Target) -> CommandResult {
        let item = CdClientObjectsId(target.id);
        let ic = item.req_item_component(d)?;

        let mut embed = object_embed(d, item);
        if let Some(description) = item.description(d) {
            embed = embed.description(description);
        }

        let locations = item.equip_locations(d);
        let proxies: Vec<String> = item.proxies(d).iter().map(|p| p.hyperlink_name(d)).collect();
        let (armor, life, imagination) = item.stat_bonuses(d);

        embed = embed
            .field("Rarity", ic.rarity.map_or_else(|| String::from("None"), |r| format!("Tier {r}")), true)
            .field(
                "Equip Location(s)",
                if locations.is_empty() { String::from("None") } else { locations.join(", ") },
                true,
            )
            .field("Proxies", if proxies.is_empty() { String::from("None") } else { proxies.join(", ") }, true);

        if armor != 0 || life != 0 || imagination != 0 {
            embed = embed
                .field("Armor", armor.to_string(), true)
                .field("Life", life.to_string(), true)
                .field("Imagination", imagination.to_string(), true);
        }

        embed = embed
            .field("Cost", format!("{} coins", format_number(ic.base_value.unwrap_or(0).into())), true)
            .field("Stack Size", ic.stack_size.unwrap_or(1).max(1).to_string(), true)
            .field(
                "Level Requirement",
                item.level_requirement(d).map_or_else(|| String::from("None"), |l| l.to_string()),
                true,
            );

        if locations.iter().any(|l| l == "Right Hand") && locations.iter().any(|l| l == "Left Hand") {
            embed = embed.field("Two Handed", "Yes", true);
        }
        if ic.is_unique {
            embed = embed.field("Unique", "Only one can be held", true);
        }
        if ic.is_bop {
            embed = embed.field("Binding", "Binds on pickup", true);
        } else if ic.is_boe {
            embed = embed.field("Binding", "Binds on equip", true);
        }

        let sets: Vec<String> = item
            .item_sets(d)
            .iter()
            .map(|&set| {
                let name = d
                    .locale
                    .item_sets
                    .get(&set)
                    .and_then(|s| s.kit_name.clone())
                    .unwrap_or_else(|| format!("Item Set {set}"));
                let rank = d
                    .cdclient
                    .item_sets
                    .at_key(&set)
                    .and_then(|s| s.kit_rank)
                    .filter(|&r| r > 0)
                    .map(|r| format!(" (Rank {r})"))
                    .unwrap_or_default();
                format!("{}{rank} `[{set}]`", crate::ids::escape_markdown(&name))
            })
            .collect();
        if !sets.is_empty() {
            embed = embed.field("Item Set", sets.join("\n"), false);
        }

        Ok(Response::new(embed).rows(item_rows(d, item, Self::NAME)))
    }
}
