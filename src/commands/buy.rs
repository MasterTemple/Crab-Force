use crate::cdclient::components::VENDOR_COMPONENT;
use crate::data::GameData;
use crate::ids::{CdClientObjectsId, LUExplorer};
use crate::interaction_command::{select_row, CommandResult, InteractionCommand, OptionKind, Response, Target};
use crate::loot::format_number;
use crate::pager::Pager;
use crate::search::SearchCategory;

use super::{item_rows, numbered, object_embed, object_select_option};

/// Sells items for faction tokens when no vendor lists the item
const HONOR_ACCOLADE: i32 = 13806;

pub struct BuyCommand;

/// The coin price at a vendor, which scales the item's base value
pub fn vendor_price(base_value: i32, buy_scalar: f64) -> i64 {
    (base_value as f64 * buy_scalar).floor() as i64
}

impl InteractionCommand for BuyCommand {
    const NAME: &'static str = "buy";
    const DESCRIPTION: &'static str = "View all vendors that sell an item!";
    const OPTION_NAME: &'static str = "item";
    const OPTION_DESCRIPTION: &'static str = "An item in LEGO Universe (name or id).";
    const OPTION: OptionKind = OptionKind::Search(SearchCategory::Item);

    fn run(d: &GameData, target: Target) -> CommandResult {
        let item = CdClientObjectsId(target.id);
        let ic = item.req_item_component(d)?;
        let base_value = ic.base_value.unwrap_or(0);

        let mut extra_costs = vec![];
        if let (Some(cost), Some(lot)) = (ic.commendation_cost.filter(|&c| c > 0), ic.commendation_lot) {
            extra_costs.push(format!("**{}** × {}", format_number(cost.into()), CdClientObjectsId(lot).hyperlink_name(d)));
        }
        if let (Some(cost), Some(lot)) = (ic.alt_currency_cost.filter(|&c| c > 0), ic.currency_lot) {
            extra_costs.push(format!("**{}** × {}", format_number(cost.into()), CdClientObjectsId(lot).hyperlink_name(d)));
        }

        let mut vendors = item.vendors(d);
        if vendors.is_empty() && ic.commendation_cost.unwrap_or(0) > 0 && d.cdclient.objects.at_key(&HONOR_ACCOLADE).is_some() {
            vendors.push(CdClientObjectsId(HONOR_ACCOLADE));
        }

        let pager = Pager::new(vendors, target.page, 15);
        let mut embed = object_embed(d, item).title(format!("{}{}", item.name_id(d), pager.title_suffix()));
        let mut options = vec![];
        if pager.is_empty() {
            embed = embed.description(format!("{} is not sold by any vendor.", item.hyperlink_name(d)));
        } else {
            let lines: Vec<String> = pager
                .this_page()
                .map(|(num, vendor)| {
                    let scalar = vendor
                        .component_id(d, VENDOR_COMPONENT)
                        .and_then(|comp| d.cdclient.vendor_component.at_key(&comp))
                        .map_or(1.0, |vc| vc.buy_scalar);
                    let mut costs = vec![];
                    if base_value > 0 {
                        costs.push(format!("**{}** coins", format_number(vendor_price(base_value, scalar))));
                    }
                    costs.extend(extra_costs.iter().cloned());
                    options.extend(object_select_option(d, *vendor, None));
                    if costs.is_empty() {
                        numbered(num, vendor.hyperlink_name(d))
                    } else {
                        numbered(num, format!("{} for {}", vendor.hyperlink_name(d), costs.join(" + ")))
                    }
                })
                .collect();
            embed = embed.field_lines("Sold By", &lines, false);
        }

        Ok(Response::new(embed)
            .row(select_row(Self::NAME, "Open a vendor…", options))
            .row(Self::page_row(&pager, target))
            .rows(item_rows(d, item, Self::NAME)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prices() {
        assert_eq!(vendor_price(100, 1.0), 100);
        assert_eq!(vendor_price(101, 1.5), 151);
    }
}
