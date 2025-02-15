use crate::cdclient::components::{ITEM_COMPONENT, RENDER_COMPONENT};
use crate::interaction_command::{CommandResult, CustomIdOptions, InteractionCommand, ToCustomId};
use crate::queries::{AutocompleteQueries, ObjectQueries};
use crate::{int_option, CD_CLIENT, CONFIG, LOCALE_XML};
use serenity::all::{
    AutocompleteChoice, CommandOptionType, CreateActionRow, CreateCommandOption, CreateEmbed,
    ResolvedOption,
};

pub fn fix_icon_asset(asset: &str) -> String {
    asset
        .replace("\\", "/")
        .replace("../", "")
        .replace("./", "")
}

pub fn icon_asset_as_url(asset: &str) -> String {
    CONFIG.explorer_res_uri(&fix_icon_asset(asset))
}

pub struct PreconditionsCommand;

pub struct PreconditionsArguments {
    item: i32,
}

impl ToCustomId for PreconditionsArguments {
    const CMD: &'static str = PreconditionsCommand::NAME;

    fn parameters(&self) -> String {
        let PreconditionsArguments { item } = self;
        format!("item={item}")
    }
}

impl<'a> TryFrom<CustomIdOptions<'a>> for PreconditionsArguments {
    type Error = String;

    fn try_from(options: CustomIdOptions<'a>) -> Result<Self, Self::Error> {
        Ok(PreconditionsArguments {
            item: options.parse("item")?,
        })
    }
}

impl<'a> TryFrom<&'a [ResolvedOption<'a>]> for PreconditionsArguments {
    type Error = String;

    fn try_from(options: &'a [ResolvedOption<'a>]) -> Result<Self, Self::Error> {
        Ok(PreconditionsArguments {
            item: int_option!(options, "item"),
        })
    }
}

impl InteractionCommand for PreconditionsCommand {
    const NAME: &'static str = "preconditions";

    const DESCRIPTION: &'static str = "View the preconditions to use an item!";

    fn options() -> Option<Vec<CreateCommandOption>> {
        Some(vec![CreateCommandOption::new(
            CommandOptionType::Integer,
            "item",
            "An item in LEGO Universe.",
        )
        .required(true)
        .set_autocomplete(true)])
    }

    type Arguments = PreconditionsArguments;

    fn handle_autocomplete(
        autocomplete_option: serenity::model::prelude::AutocompleteOption<'_>,
    ) -> Option<Vec<serenity::all::AutocompleteChoice>> {
        let input = autocomplete_option.value;
        Some(CD_CLIENT.autocomplete_object(input))
    }

    fn run(arguments: Self::Arguments) -> CommandResult {
        let PreconditionsArguments { item: item_id } = arguments;

        let explorer_url = CD_CLIENT.object_explorer_url(item_id);
        // let item = CD_CLIENT.get_object(item_id)?;
        let name = CD_CLIENT.req_object_name(item_id);
        let item_component = CD_CLIENT.object_item_component(item_id)?;

        let preconditions_map = &LOCALE_XML
            .locales
            .get(&CONFIG.locale)
            .unwrap()
            .preconditions;

        let precondition_text = if let Some(reqs) = item_component.req_precondition.as_ref() {
            reqs.iter()
                .enumerate()
                .map(|(idx, id)| {
                    let cond = preconditions_map
                        .get(&id)
                        .map(|req| req.failure_reason.clone())
                        .flatten()
                        .unwrap_or_else(|| format!("Precondition {id}"));
                    format!("**{}.** {}", idx + 1, cond)
                })
                .collect::<Vec<_>>()
                .join("\n")
        } else {
            String::from("None")
        };

        let mut embed = CONFIG
            .default_embed()
            .title(format!("{} [{}]", name, item_id))
            .url(explorer_url)
            .field("Preconditions", precondition_text, false);

        if let Some(icon_url) = CD_CLIENT.object_icon_url(item_id) {
            embed = embed.thumbnail(icon_url);
        }

        Ok((embed, None))
    }
}
