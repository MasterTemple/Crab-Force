use crate::interaction_command::{CommandResult, CustomIdOptions, InteractionCommand, ToCustomId};
use crate::pager::START_PAGE;
use crate::queries::{AutocompleteQueries, ObjectQueries};
use crate::{int_option, CD_CLIENT, CONFIG, LOCALE_XML};
use serenity::all::{AutocompleteChoice, CommandOptionType, CreateCommandOption, ResolvedOption};

pub struct BuyCommand;

pub struct BuyArguments {
    pub item: i32,
    pub page: usize,
}

impl ToCustomId for BuyArguments {
    const CMD: &'static str = BuyCommand::NAME;

    fn parameters(&self) -> String {
        let BuyArguments { item, page } = self;
        vec![format!("item={item}"), format!("page={page}")].join("&")
    }
}

impl TryFrom<&CustomIdOptions> for BuyArguments {
    type Error = String;

    fn try_from(options: &CustomIdOptions) -> Result<Self, Self::Error> {
        Ok(BuyArguments {
            item: options.parse("item")?,
            page: options.parse("page")?,
        })
    }
}

impl<'a> TryFrom<&'a [ResolvedOption<'a>]> for BuyArguments {
    type Error = String;

    fn try_from(options: &'a [ResolvedOption<'a>]) -> Result<Self, Self::Error> {
        Ok(BuyArguments {
            item: int_option!(options, "item"),
            page: START_PAGE,
        })
    }
}

impl InteractionCommand for BuyCommand {
    const NAME: &'static str = "buy";

    const DESCRIPTION: &'static str = "View all vendors that sell an item!";

    fn options() -> Option<Vec<CreateCommandOption>> {
        Some(vec![CreateCommandOption::new(
            CommandOptionType::Integer,
            "item",
            "An item in LEGO Universe.",
        )
        .required(true)
        .set_autocomplete(true)])
    }

    type Arguments = BuyArguments;

    fn handle_autocomplete(
        autocomplete_option: serenity::model::prelude::AutocompleteOption<'_>,
    ) -> Option<Vec<serenity::all::AutocompleteChoice>> {
        let input = autocomplete_option.value;
        Some(CD_CLIENT.autocomplete_object(input))
    }

    fn run(arguments: Self::Arguments) -> CommandResult {
        let BuyArguments {
            item: item_id,
            page,
        } = arguments;

        let explorer_url = CD_CLIENT.object_explorer_url(item_id);
        let name = CD_CLIENT.req_object_name(item_id);
        let item_component = CD_CLIENT.object_item_component(item_id)?;

        let coin_cost = item_component
            .base_value
            .map(|cost| format!("**{cost}** coins"));

        let commendation_cost = item_component.commendation_cost.map(|cost| {
            let lot = item_component.commendation_lot.ok_or_else(|| {
                format!("Commendation Cost Provided but Commendation Object is not")
            });
            lot.map(|lot| {
                let name = CD_CLIENT.object_hyperlinked_name(lot);
                format!("**{cost}** {name}")
            })
        });

        let alt_cost = item_component.alt_currency_cost.map(|cost| {
            let lot = item_component.currency_lot.ok_or_else(|| {
                format!("Alternate Currency Cost Provided but Alternate Currency Object is not")
            });
            lot.map(|lot| {
                let name = CD_CLIENT.object_hyperlinked_name(lot);
                format!("**{cost}** {name}")
            })
        });

        let mut embed = CONFIG
            .default_embed()
            .title(format!("{} [{}]", name, item_id))
            .url(explorer_url);

        let vendors = CD_CLIENT
            .object_vendor_ids(item_id)
            .unwrap_or_else(|_| vec![]);
        let description = if vendors.len() == 0 {
            format!("This item is not sold")
        } else {
            let vendors = vendors
                .into_iter()
                .enumerate()
                .map(|(idx, vendor_id)| {
                    let name = CD_CLIENT.object_hyperlinked_name(vendor_id);
                    format!("{}. {name}", idx + 1)
                })
                .collect::<Vec<_>>()
                .join("\n");
            format!("**Vendors**\n{vendors}")
        };
        embed = embed.description(description);

        if let Some(coin_cost) = coin_cost {
            embed = embed.field("Coins", coin_cost, true);
        }

        if let Some(commendation_cost) = commendation_cost {
            embed = embed.field("Faction Token Cost", commendation_cost?, true);
        }

        if let Some(alt_cost) = alt_cost {
            embed = embed.field("Alternate Currency", alt_cost?, true);
        }

        if let Some(icon_url) = CD_CLIENT.object_icon_url(item_id) {
            embed = embed.thumbnail(icon_url);
        }

        Ok((embed, None))
    }
}
