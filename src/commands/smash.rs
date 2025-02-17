use crate::ids::CdClientObjectsId;
use crate::interaction_command::{CommandResult, CustomIdOptions, InteractionCommand, ToCustomId};
use crate::queries::{AutocompleteQueries, ObjectQueries};
use crate::{int_option, CD_CLIENT, CONFIG, LOCALE_XML};
use serenity::all::{
    AutocompleteChoice, CommandOptionType, CreateCommandOption, CreateSelectMenuOption,
    ResolvedOption,
};

pub struct SmashCommand;

pub struct SmashArguments {
    pub smashable: i32,
}

impl ToCustomId for SmashArguments {
    const CMD: &'static str = SmashCommand::NAME;

    fn parameters(&self) -> String {
        let SmashArguments { smashable } = self;
        format!("smashable={smashable}")
    }
}

impl TryFrom<&CustomIdOptions> for SmashArguments {
    type Error = String;

    fn try_from(options: &CustomIdOptions) -> Result<Self, Self::Error> {
        Ok(SmashArguments {
            smashable: options.parse("smashable")?,
        })
    }
}

impl<'a> TryFrom<&'a [ResolvedOption<'a>]> for SmashArguments {
    type Error = String;

    fn try_from(options: &'a [ResolvedOption<'a>]) -> Result<Self, Self::Error> {
        Ok(SmashArguments {
            smashable: int_option!(options, "smashable"),
        })
    }
}

impl Into<CreateSelectMenuOption> for SmashArguments {
    fn into(self) -> CreateSelectMenuOption {
        let it = CdClientObjectsId(self.smashable);
        CreateSelectMenuOption::new(it.name_id(), self.to_custom_id(true))
    }
}

impl InteractionCommand for SmashCommand {
    const NAME: &'static str = "smashable";

    const DESCRIPTION: &'static str = "View a smashable in LEGO Universe!";

    fn options() -> Option<Vec<CreateCommandOption>> {
        Some(vec![CreateCommandOption::new(
            CommandOptionType::Integer,
            "smashable",
            "A smashable in LEGO Universe.",
        )
        .required(true)
        .set_autocomplete(true)])
    }

    type Arguments = SmashArguments;

    fn handle_autocomplete(
        autocomplete_option: serenity::model::prelude::AutocompleteOption<'_>,
    ) -> Option<Vec<serenity::all::AutocompleteChoice>> {
        let input = autocomplete_option.value;
        Some(CD_CLIENT.autocomplete_object(input))
    }

    fn run(arguments: Self::Arguments) -> CommandResult {
        let SmashArguments { smashable: id } = arguments;

        let explorer_url = CD_CLIENT.object_explorer_url(id);
        let name = CD_CLIENT.req_object_name(id);
        // let item_component = CD_CLIENT.object_item_component(id)?;

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
