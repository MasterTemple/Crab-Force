use serenity::all::{ButtonStyle, CreateActionRow, CreateButton};

use crate::data::GameData;
use crate::embed::Embed;
use crate::ids::{CdClientLootTableId, LUExplorer};
use crate::interaction_command::{select_row, CommandResult, CustomId, InteractionCommand, Mode, OptionKind, Response, Target, MAX_BUTTONS};
use crate::loot::items_by_rarity;
use crate::pager::{Pager, START_PAGE};
use crate::search::SearchCategory;

use super::item::ItemCommand;
use super::numbered;

/// The tier used for items without a rarity
const NO_RARITY: i32 = 0;

pub struct LootTableCommand;

impl InteractionCommand for LootTableCommand {
    const NAME: &'static str = "loottable";
    const DESCRIPTION: &'static str = "View all items in a loot table!";
    const OPTION_NAME: &'static str = "loottable";
    const OPTION_DESCRIPTION: &'static str = "A loot table (name or id).";
    const OPTION: OptionKind = OptionKind::Search(SearchCategory::LootTable);

    fn run(d: &GameData, target: Target) -> CommandResult {
        let table = CdClientLootTableId(target.id);
        if !table.exists(d) {
            return Err(table.not_found(d));
        }
        let by_rarity = items_by_rarity(d, table);
        let tiers: Vec<i32> = by_rarity.keys().map(|r| r.unwrap_or(NO_RARITY)).collect();
        // default to the lowest tier that has items
        let tier = target
            .tier
            .filter(|t| tiers.contains(t))
            .or_else(|| tiers.iter().copied().find(|&t| t != NO_RARITY))
            .or_else(|| tiers.first().copied())
            .unwrap_or(NO_RARITY);
        let rarity = (tier != NO_RARITY).then_some(tier);
        let items = by_rarity.get(&rarity).cloned().unwrap_or_default();
        let total: usize = by_rarity.values().map(Vec::len).sum();

        let pager = Pager::new(items, target.page, 20);
        let tier_name = if tier == NO_RARITY { String::from("No Rarity") } else { format!("Tier {tier}") };
        let mut embed = Embed::new()
            .title(format!("{}{}", table.name_id(d), pager.title_suffix()))
            .url(table.explorer_url())
            .description(format!(
                "{total} item{} in this table. Used by {} loot matri{}.",
                if total == 1 { "" } else { "s" },
                d.index.loot_matrices_with_table.get(&table.0).map_or(0, Vec::len),
                if d.index.loot_matrices_with_table.get(&table.0).map_or(0, Vec::len) == 1 { "x" } else { "ces" },
            ));
        if let Some(first) = pager.this_page().next() {
            embed = embed.thumbnail(first.1.thumbnail(d));
        }

        let mut options = vec![];
        let lines: Vec<String> = pager
            .this_page()
            .map(|(num, item)| {
                options.push(ItemCommand::select_option(item.name_id(d), None, Target::new(item.0)));
                numbered(num, item.hyperlink_name(d))
            })
            .collect();
        embed = embed.field_lines(format!("{tier_name} ({} items)", pager.total()), &lines, false);

        let tier_buttons: Vec<CreateButton> = tiers
            .iter()
            .take(MAX_BUTTONS)
            .map(|&t| {
                let label = if t == NO_RARITY { String::from("No Rarity") } else { format!("Tier {t}") };
                CreateButton::new(CustomId::encode(Mode::Update, Self::NAME, Target::new(table.0).page(START_PAGE).tier(Some(t)), Some("tier")))
                    .label(label)
                    .style(if t == tier { ButtonStyle::Success } else { ButtonStyle::Primary })
                    .disabled(t == tier)
            })
            .collect();
        let tier_row = (tier_buttons.len() > 1).then_some(CreateActionRow::Buttons(tier_buttons));

        Ok(Response::new(embed)
            .row(select_row(Self::NAME, "Open an item…", options))
            .row(tier_row)
            .row(Self::page_row(&pager, Target::new(table.0).tier(Some(tier)))))
    }
}
