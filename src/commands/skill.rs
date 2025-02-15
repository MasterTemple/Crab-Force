use crate::interaction_command::{CommandResult, CustomIdOptions, InteractionCommand, ToCustomId};
use crate::queries::{AutocompleteQueries, ObjectQueries};
use crate::{int_option, CD_CLIENT, CONFIG, LOCALE_XML};
use serenity::all::{AutocompleteChoice, CommandOptionType, CreateCommandOption, ResolvedOption};

pub struct SkillCommand;

pub struct SkillArguments {
    skill: i32,
}

impl ToCustomId for SkillArguments {
    const CMD: &'static str = SkillCommand::NAME;

    fn parameters(&self) -> String {
        let SkillArguments { skill } = self;
        format!("skill={skill}")
    }
}

impl<'a> TryFrom<CustomIdOptions<'a>> for SkillArguments {
    type Error = String;

    fn try_from(options: CustomIdOptions<'a>) -> Result<Self, Self::Error> {
        Ok(SkillArguments {
            skill: options.parse("skill")?,
        })
    }
}

impl<'a> TryFrom<&'a [ResolvedOption<'a>]> for SkillArguments {
    type Error = String;

    fn try_from(options: &'a [ResolvedOption<'a>]) -> Result<Self, Self::Error> {
        Ok(SkillArguments {
            skill: int_option!(options, "skill"),
        })
    }
}

impl InteractionCommand for SkillCommand {
    const NAME: &'static str = "skill";

    const DESCRIPTION: &'static str = "View a skill in LEGO Universe!";

    fn options() -> Option<Vec<CreateCommandOption>> {
        Some(vec![CreateCommandOption::new(
            CommandOptionType::Integer,
            "skill",
            "A skill in LEGO Universe.",
        )
        .required(true)
        .set_autocomplete(true)])
    }

    type Arguments = SkillArguments;

    fn handle_autocomplete(
        autocomplete_option: serenity::model::prelude::AutocompleteOption<'_>,
    ) -> Option<Vec<serenity::all::AutocompleteChoice>> {
        let input = autocomplete_option.value;
        Some(CD_CLIENT.autocomplete_object(input))
    }

    fn run(arguments: Self::Arguments) -> CommandResult {
        let SkillArguments { skill: id } = arguments;

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
