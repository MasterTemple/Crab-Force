use crate::interaction_command::{CommandResult, CustomIdOptions, InteractionCommand, ToCustomId};
use crate::queries::{AutocompleteQueries, ObjectQueries};
use crate::{int_option, CD_CLIENT, CONFIG, LOCALE_XML};
use serenity::all::{AutocompleteChoice, CommandOptionType, CreateCommandOption, ResolvedOption};

pub struct BrickCommand;

pub struct BrickArguments {
    brick: i32,
}

impl ToCustomId for BrickArguments {
    const CMD: &'static str = BrickCommand::NAME;

    fn parameters(&self) -> String {
        let BrickArguments { brick } = self;
        format!("brick={brick}")
    }
}

impl TryFrom<&CustomIdOptions> for BrickArguments {
    type Error = String;

    fn try_from(options: &CustomIdOptions) -> Result<Self, Self::Error> {
        Ok(BrickArguments {
            brick: options.parse("brick")?,
        })
    }
}

impl<'a> TryFrom<&'a [ResolvedOption<'a>]> for BrickArguments {
    type Error = String;

    fn try_from(options: &'a [ResolvedOption<'a>]) -> Result<Self, Self::Error> {
        Ok(BrickArguments {
            brick: int_option!(options, "brick"),
        })
    }
}

impl InteractionCommand for BrickCommand {
    const NAME: &'static str = "brick";

    const DESCRIPTION: &'static str = "View the stats of a brick!";

    fn options() -> Option<Vec<CreateCommandOption>> {
        Some(vec![CreateCommandOption::new(
            CommandOptionType::Integer,
            "brick",
            "A brick in LEGO Universe.",
        )
        .required(true)
        .set_autocomplete(true)])
    }

    type Arguments = BrickArguments;

    fn handle_autocomplete(
        autocomplete_option: serenity::model::prelude::AutocompleteOption<'_>,
    ) -> Option<Vec<serenity::all::AutocompleteChoice>> {
        let input = autocomplete_option.value;
        Some(CD_CLIENT.autocomplete_object(input))
    }

    fn run(arguments: Self::Arguments) -> CommandResult {
        let BrickArguments { brick: id } = arguments;

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
