use crate::interaction_command::{CommandResult, CustomIdOptions, InteractionCommand, ToCustomId};
use crate::queries::{AutocompleteQueries, ObjectQueries};
use crate::{int_option, CD_CLIENT, CONFIG, LOCALE_XML};
use serenity::all::{AutocompleteChoice, CommandOptionType, CreateCommandOption, ResolvedOption};

pub struct MissionCommand;

pub struct MissionArguments {
    mission: i32,
}

impl ToCustomId for MissionArguments {
    const CMD: &'static str = MissionCommand::NAME;

    fn parameters(&self) -> String {
        let MissionArguments { mission } = self;
        format!("mission={mission}")
    }
}

impl<'a> TryFrom<CustomIdOptions<'a>> for MissionArguments {
    type Error = String;

    fn try_from(options: CustomIdOptions<'a>) -> Result<Self, Self::Error> {
        Ok(MissionArguments {
            mission: options.parse("mission")?,
        })
    }
}

impl<'a> TryFrom<&'a [ResolvedOption<'a>]> for MissionArguments {
    type Error = String;

    fn try_from(options: &'a [ResolvedOption<'a>]) -> Result<Self, Self::Error> {
        Ok(MissionArguments {
            mission: int_option!(options, "mission"),
        })
    }
}

impl InteractionCommand for MissionCommand {
    const NAME: &'static str = "mission";

    const DESCRIPTION: &'static str = "View the stats of an mission!";

    fn options() -> Option<Vec<CreateCommandOption>> {
        Some(vec![CreateCommandOption::new(
            CommandOptionType::Integer,
            "mission",
            "An mission in LEGO Universe.",
        )
        .required(true)
        .set_autocomplete(true)])
    }

    type Arguments = MissionArguments;

    fn handle_autocomplete(
        autocomplete_option: serenity::model::prelude::AutocompleteOption<'_>,
    ) -> Option<Vec<serenity::all::AutocompleteChoice>> {
        let input = autocomplete_option.value;
        Some(CD_CLIENT.autocomplete_object(input))
    }

    fn run(arguments: Self::Arguments) -> CommandResult {
        let MissionArguments { mission: id } = arguments;

        let explorer_url = CD_CLIENT.object_explorer_url(id);
        let name = CD_CLIENT.req_object_name(id);
        let item_component = CD_CLIENT.object_item_component(id)?;

        let mut embed = CONFIG
            .default_embed()
            .title(format!("{} [{}]", name, id))
            .url(explorer_url);

        Ok((embed, None))
    }
}
