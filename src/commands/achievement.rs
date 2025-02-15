use crate::interaction_command::{CommandResult, CustomIdOptions, InteractionCommand, ToCustomId};
use crate::queries::{AutocompleteQueries, ObjectQueries};
use crate::{int_option, CD_CLIENT, CONFIG, LOCALE_XML};
use serenity::all::{AutocompleteChoice, CommandOptionType, CreateCommandOption, ResolvedOption};

pub struct AchievementCommand;

pub struct AchievementArguments {
    achievement: i32,
}

impl ToCustomId for AchievementArguments {
    const CMD: &'static str = AchievementCommand::NAME;

    fn parameters(&self) -> String {
        let AchievementArguments { achievement } = self;
        format!("achievement={achievement}")
    }
}

impl<'a> TryFrom<CustomIdOptions<'a>> for AchievementArguments {
    type Error = String;

    fn try_from(options: CustomIdOptions<'a>) -> Result<Self, Self::Error> {
        Ok(AchievementArguments {
            achievement: options.parse("achievement")?,
        })
    }
}

impl<'a> TryFrom<&'a [ResolvedOption<'a>]> for AchievementArguments {
    type Error = String;

    fn try_from(options: &'a [ResolvedOption<'a>]) -> Result<Self, Self::Error> {
        Ok(AchievementArguments {
            achievement: int_option!(options, "achievement"),
        })
    }
}

impl InteractionCommand for AchievementCommand {
    const NAME: &'static str = "achievement";

    const DESCRIPTION: &'static str = "View the stats of an achievement!";

    fn options() -> Option<Vec<CreateCommandOption>> {
        Some(vec![CreateCommandOption::new(
            CommandOptionType::Integer,
            "achievement",
            "An achievement in LEGO Universe.",
        )
        .required(true)
        .set_autocomplete(true)])
    }

    type Arguments = AchievementArguments;

    fn handle_autocomplete(
        autocomplete_option: serenity::model::prelude::AutocompleteOption<'_>,
    ) -> Option<Vec<serenity::all::AutocompleteChoice>> {
        let input = autocomplete_option.value;
        Some(CD_CLIENT.autocomplete_object(input))
    }

    fn run(arguments: Self::Arguments) -> CommandResult {
        let AchievementArguments { achievement: id } = arguments;

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
