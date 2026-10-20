use crate::cdclient::components::VENDOR_COMPONENT;
use crate::data::GameData;
use crate::error::BotError;
use crate::ids::{CdClientLootTableId, CdClientObjectsId, LUExplorer};
use crate::interaction_command::{select_row, CommandResult, InteractionCommand, OptionKind, Response, Target};
use crate::loot::format_number;
use crate::pager::Pager;
use crate::search::SearchCategory;

use super::buy::vendor_price;
use super::item::ItemCommand;
use super::{npc_row, numbered, object_embed};

pub struct VendorCommand;

impl InteractionCommand for VendorCommand {
    const NAME: &'static str = "vendor";
    const DESCRIPTION: &'static str = "View all items sold by a vendor!";
    const OPTION_NAME: &'static str = "vendor";
    const OPTION_DESCRIPTION: &'static str = "A vendor in LEGO Universe (name or id).";
    const OPTION: OptionKind = OptionKind::Search(SearchCategory::Vendor);

    fn run(d: &GameData, target: Target) -> CommandResult {
        let vendor = CdClientObjectsId(target.id);
        vendor.try_fetch(d)?;
        let lmi = vendor
            .vendor_loot_matrix(d)
            .ok_or_else(|| BotError::not_applicable(format!("{} does not sell anything.", vendor.hyperlink_name(d))))?;
        let scalar = vendor
            .component_id(d, VENDOR_COMPONENT)
            .and_then(|comp| d.cdclient.vendor_component.at_key(&comp))
            .map_or(1.0, |vc| vc.buy_scalar);

        let mut items: Vec<CdClientObjectsId> = vec![];
        for row in d.cdclient.loot_matrix.at_group_key(&lmi).unwrap_or_default() {
            for item in CdClientLootTableId(row.loot_table_index).items(d) {
                if !items.contains(&item) {
                    items.push(item);
                }
            }
        }
        let pager = Pager::new(items, target.page, 20);

        let mut embed = object_embed(d, vendor).title(format!("{}{}", vendor.name_id(d), pager.title_suffix()));
        let mut options = vec![];
        if pager.is_empty() {
            embed = embed.description(format!("{} does not sell anything.", vendor.hyperlink_name(d)));
        } else {
            let lines: Vec<String> = pager
                .this_page()
                .map(|(num, &item)| {
                    let mut costs = vec![];
                    if let Some(ic) = item.item_component(d) {
                        if let Some(value) = ic.base_value.filter(|&v| v > 0) {
                            costs.push(format!("**{}** coins", format_number(vendor_price(value, scalar))));
                        }
                        for (cost, lot) in [(ic.commendation_cost, ic.commendation_lot), (ic.alt_currency_cost, ic.currency_lot)] {
                            if let (Some(cost), Some(lot)) = (cost.filter(|&c| c > 0), lot) {
                                costs.push(format!("**{}** × {}", format_number(cost.into()), CdClientObjectsId(lot).req_name(d)));
                            }
                        }
                    }
                    options.push(ItemCommand::select_option(item.name_id(d), None, Target::new(item.0)));
                    let cost = if costs.is_empty() { String::new() } else { format!(" for {}", costs.join(" + ")) };
                    numbered(num, format!("{}{cost}", item.hyperlink_name(d)))
                })
                .collect();
            embed = embed.field_lines(format!("Sells {} Items", pager.total()), &lines, false);
        }

        Ok(Response::new(embed)
            .row(select_row(Self::NAME, "Open an item…", options))
            .row(Self::page_row(&pager, target))
            .row(Some(npc_row(d, vendor, Self::NAME))))
    }
}
