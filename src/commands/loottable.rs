use crate::interaction_command::{CommandResult, CustomIdOptions, InteractionCommand, ToCustomId};
use crate::queries::{AutocompleteQueries, ObjectQueries};
use crate::{int_option, CD_CLIENT, CONFIG, LOCALE_XML};
use serenity::all::{AutocompleteChoice, CommandOptionType, CreateCommandOption, ResolvedOption};

pub struct LootTableCommand;

pub struct LootTableArguments {
    table: i32,
}

impl ToCustomId for LootTableArguments {
    const CMD: &'static str = LootTableCommand::NAME;

    fn parameters(&self) -> String {
        let LootTableArguments { table } = self;
        format!("table={table}")
    }
}

impl TryFrom<&CustomIdOptions> for LootTableArguments {
    type Error = String;

    fn try_from(options: &CustomIdOptions) -> Result<Self, Self::Error> {
        Ok(LootTableArguments {
            table: options.parse("table")?,
        })
    }
}

impl<'a> TryFrom<&'a [ResolvedOption<'a>]> for LootTableArguments {
    type Error = String;

    fn try_from(options: &'a [ResolvedOption<'a>]) -> Result<Self, Self::Error> {
        Ok(LootTableArguments {
            table: int_option!(options, "table"),
        })
    }
}

impl InteractionCommand for LootTableCommand {
    const NAME: &'static str = "table";

    const DESCRIPTION: &'static str = "View the stats of a loot table!";

    fn options() -> Option<Vec<CreateCommandOption>> {
        Some(vec![CreateCommandOption::new(
            CommandOptionType::Integer,
            "table",
            "A loot table in LEGO Universe.",
        )
        .required(true)
        .set_autocomplete(true)])
    }

    type Arguments = LootTableArguments;

    fn handle_autocomplete(
        autocomplete_option: serenity::model::prelude::AutocompleteOption<'_>,
    ) -> Option<Vec<serenity::all::AutocompleteChoice>> {
        let input = autocomplete_option.value;
        Some(CD_CLIENT.autocomplete_object(input))
    }

    fn run(arguments: Self::Arguments) -> CommandResult {
        let LootTableArguments { table: id } = arguments;

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
