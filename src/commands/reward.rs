use crate::interaction_command::{CommandResult, CustomIdOptions, InteractionCommand, ToCustomId};
use crate::pager::START_PAGE;
use crate::queries::{AutocompleteQueries, ObjectQueries};
use crate::{int_option, CD_CLIENT, CONFIG, LOCALE_XML};
use serenity::all::{AutocompleteChoice, CommandOptionType, CreateCommandOption, ResolvedOption};

pub struct RewardCommand;

pub struct RewardArguments {
    pub item: i32,
    pub page: usize,
}

impl ToCustomId for RewardArguments {
    const CMD: &'static str = RewardCommand::NAME;

    fn parameters(&self) -> String {
        let RewardArguments { item, page } = self;
        vec![format!("item={item}"), format!("page={page}")].join("&")
    }
}

impl TryFrom<&CustomIdOptions> for RewardArguments {
    type Error = String;

    fn try_from(options: &CustomIdOptions) -> Result<Self, Self::Error> {
        Ok(RewardArguments {
            item: options.parse("item")?,
            page: options.parse("page")?,
        })
    }
}

impl<'a> TryFrom<&'a [ResolvedOption<'a>]> for RewardArguments {
    type Error = String;

    fn try_from(options: &'a [ResolvedOption<'a>]) -> Result<Self, Self::Error> {
        Ok(RewardArguments {
            item: int_option!(options, "item"),
            page: START_PAGE,
        })
    }
}

impl InteractionCommand for RewardCommand {
    const NAME: &'static str = "reward";

    const DESCRIPTION: &'static str = "View all activities that drop an item!";

    fn options() -> Option<Vec<CreateCommandOption>> {
        Some(vec![CreateCommandOption::new(
            CommandOptionType::Integer,
            "item",
            "An item in LEGO Universe.",
        )
        .required(true)
        .set_autocomplete(true)])
    }

    type Arguments = RewardArguments;

    fn handle_autocomplete(
        autocomplete_option: serenity::model::prelude::AutocompleteOption<'_>,
    ) -> Option<Vec<serenity::all::AutocompleteChoice>> {
        let input = autocomplete_option.value;
        Some(CD_CLIENT.autocomplete_object(input))
    }

    fn run(arguments: Self::Arguments) -> CommandResult {
        let RewardArguments { item: id, page } = arguments;

        let explorer_url = CD_CLIENT.object_explorer_url(id);
        let name = CD_CLIENT.req_object_name(id);
        let item_component = CD_CLIENT.object_item_component(id)?;

        let mut embed = CONFIG
            .default_embed()
            .title(format!("{} [{}]", name, id))
            .url(explorer_url);

        if let Some(icon_url) = CD_CLIENT.object_icon_url(id) {
            embed = embed.thumbnail(icon_url);
        }

        Ok((embed, None))
    }
}
