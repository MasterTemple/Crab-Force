use crate::interaction_command::{CommandResult, CustomIdOptions, InteractionCommand, ToCustomId};
use crate::queries::{AutocompleteQueries, ObjectQueries};
use crate::{int_option, CD_CLIENT, CONFIG, LOCALE_XML};
use serenity::all::{AutocompleteChoice, CommandOptionType, CreateCommandOption, ResolvedOption};

pub struct SkillItemsCommand;

pub struct SkillItemsArguments {
    skill: i32,
}

impl ToCustomId for SkillItemsArguments {
    const CMD: &'static str = SkillItemsCommand::NAME;

    fn parameters(&self) -> String {
        let SkillItemsArguments { skill } = self;
        format!("skill={skill}")
    }
}

impl<'a> TryFrom<CustomIdOptions<'a>> for SkillItemsArguments {
    type Error = String;

    fn try_from(options: CustomIdOptions<'a>) -> Result<Self, Self::Error> {
        Ok(SkillItemsArguments {
            skill: options.parse("skill")?,
        })
    }
}

impl<'a> TryFrom<&'a [ResolvedOption<'a>]> for SkillItemsArguments {
    type Error = String;

    fn try_from(options: &'a [ResolvedOption<'a>]) -> Result<Self, Self::Error> {
        Ok(SkillItemsArguments {
            skill: int_option!(options, "skill"),
        })
    }
}

impl InteractionCommand for SkillItemsCommand {
    const NAME: &'static str = "skill_items";

    const DESCRIPTION: &'static str = "View all items that have a skill!";

    fn options() -> Option<Vec<CreateCommandOption>> {
        Some(vec![CreateCommandOption::new(
            CommandOptionType::Integer,
            "skill",
            "An skill in LEGO Universe.",
        )
        .required(true)
        .set_autocomplete(true)])
    }

    type Arguments = SkillItemsArguments;

    fn handle_autocomplete(
        autocomplete_option: serenity::model::prelude::AutocompleteOption<'_>,
    ) -> Option<Vec<serenity::all::AutocompleteChoice>> {
        let input = autocomplete_option.value;
        Some(CD_CLIENT.autocomplete_object(input))
    }

    fn run(arguments: Self::Arguments) -> CommandResult {
        let SkillItemsArguments { skill: id } = arguments;

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
