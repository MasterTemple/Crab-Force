use crate::interaction_command::{CommandResult, CustomIdOptions, InteractionCommand, ToCustomId};
use crate::queries::{AutocompleteQueries, ObjectQueries, SkillQueries};
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

impl TryFrom<&CustomIdOptions> for SkillItemsArguments {
    type Error = String;

    fn try_from(options: &CustomIdOptions) -> Result<Self, Self::Error> {
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
        Some(CD_CLIENT.autocomplete_skill(input))
    }

    fn run(arguments: Self::Arguments) -> CommandResult {
        let SkillItemsArguments { skill: id } = arguments;

        let skill = CD_CLIENT.get_skill(id)?;
        let explorer_url = CD_CLIENT.skill_explorer_url(id);
        let name = CD_CLIENT.req_skill_name(id);

        let object_skills: Vec<_> = CD_CLIENT
            .object_skills
            .iter()
            .filter(|ob| ob.skill_id == id)
            .enumerate()
            .map(|(idx, ob)| {
                let num = idx + 1;
                let name = CD_CLIENT.object_hyperlinked_name(ob.object_template);
                format!("**{num}.** {name}")
            })
            .collect();

        let mut embed = CONFIG
            .default_embed()
            .title(format!("{} [{}]", name, id))
            .url(explorer_url);

        if let Some(icon_url) = CD_CLIENT.skill_icon_url(id) {
            embed = embed.thumbnail(icon_url);
        }

        match object_skills.len() {
            0 => embed = embed.field("Items", "None", true),
            1 => embed = embed.field("Item", &object_skills[0], true),
            _ => {
                let half = object_skills.len() / 2;
                let col1 = object_skills[..=half].join("\n");
                let col2 = object_skills[half + 1..].join("\n");
                embed = embed.field("Items", col1, true);
                embed = embed.field("Items", col2, true);
            }
        };

        Ok((embed, None))
    }
}
