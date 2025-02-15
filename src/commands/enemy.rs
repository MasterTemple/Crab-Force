use crate::interaction_command::{CommandResult, CustomIdOptions, InteractionCommand, ToCustomId};
use crate::queries::{AutocompleteQueries, ObjectQueries};
use crate::{int_option, CD_CLIENT, CONFIG, LOCALE_XML};
use serenity::all::{AutocompleteChoice, CommandOptionType, CreateCommandOption, ResolvedOption};

pub struct EnemyCommand;

pub struct EnemyArguments {
    enemy: i32,
}

impl ToCustomId for EnemyArguments {
    const CMD: &'static str = EnemyCommand::NAME;

    fn parameters(&self) -> String {
        let EnemyArguments { enemy } = self;
        format!("enemy={enemy}")
    }
}

impl<'a> TryFrom<CustomIdOptions<'a>> for EnemyArguments {
    type Error = String;

    fn try_from(options: CustomIdOptions<'a>) -> Result<Self, Self::Error> {
        Ok(EnemyArguments {
            enemy: options.parse("enemy")?,
        })
    }
}

impl<'a> TryFrom<&'a [ResolvedOption<'a>]> for EnemyArguments {
    type Error = String;

    fn try_from(options: &'a [ResolvedOption<'a>]) -> Result<Self, Self::Error> {
        Ok(EnemyArguments {
            enemy: int_option!(options, "enemy"),
        })
    }
}

impl InteractionCommand for EnemyCommand {
    const NAME: &'static str = "enemy";

    const DESCRIPTION: &'static str = "View an enemy in LEGO Universe!";

    fn options() -> Option<Vec<CreateCommandOption>> {
        Some(vec![CreateCommandOption::new(
            CommandOptionType::Integer,
            "enemy",
            "An enemy in LEGO Universe.",
        )
        .required(true)
        .set_autocomplete(true)])
    }

    type Arguments = EnemyArguments;

    fn handle_autocomplete(
        autocomplete_option: serenity::model::prelude::AutocompleteOption<'_>,
    ) -> Option<Vec<serenity::all::AutocompleteChoice>> {
        let input = autocomplete_option.value;
        Some(CD_CLIENT.autocomplete_object(input))
    }

    fn run(arguments: Self::Arguments) -> CommandResult {
        let EnemyArguments { enemy: id } = arguments;

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
