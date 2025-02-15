use crate::interaction_command::{CommandResult, CustomIdOptions, InteractionCommand, ToCustomId};
use crate::queries::{AutocompleteQueries, ObjectQueries};
use crate::{int_option, CD_CLIENT, CONFIG, LOCALE_XML};
use serenity::all::{AutocompleteChoice, CommandOptionType, CreateCommandOption, ResolvedOption};

pub struct CooldownGroupCommand;

pub struct CooldownGroupArguments {
    group: i32,
}

impl ToCustomId for CooldownGroupArguments {
    const CMD: &'static str = CooldownGroupCommand::NAME;

    fn parameters(&self) -> String {
        let CooldownGroupArguments { group } = self;
        format!("group={group}")
    }
}

impl<'a> TryFrom<CustomIdOptions<'a>> for CooldownGroupArguments {
    type Error = String;

    fn try_from(options: CustomIdOptions<'a>) -> Result<Self, Self::Error> {
        Ok(CooldownGroupArguments {
            group: options.parse("group")?,
        })
    }
}

impl<'a> TryFrom<&'a [ResolvedOption<'a>]> for CooldownGroupArguments {
    type Error = String;

    fn try_from(options: &'a [ResolvedOption<'a>]) -> Result<Self, Self::Error> {
        Ok(CooldownGroupArguments {
            group: int_option!(options, "group"),
        })
    }
}

impl InteractionCommand for CooldownGroupCommand {
    const NAME: &'static str = "group";

    const DESCRIPTION: &'static str = "View the stats of a cooldown group!";

    fn options() -> Option<Vec<CreateCommandOption>> {
        Some(vec![CreateCommandOption::new(
            CommandOptionType::Integer,
            "group",
            "A cooldown group in LEGO Universe.",
        )
        .required(true)
        .set_autocomplete(true)])
    }

    type Arguments = CooldownGroupArguments;

    fn handle_autocomplete(
        autocomplete_option: serenity::model::prelude::AutocompleteOption<'_>,
    ) -> Option<Vec<serenity::all::AutocompleteChoice>> {
        let input = autocomplete_option.value;
        Some(CD_CLIENT.autocomplete_object(input))
    }

    fn run(arguments: Self::Arguments) -> CommandResult {
        let CooldownGroupArguments { group: id } = arguments;

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
