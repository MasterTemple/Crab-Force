use crate::interaction_command::{CommandResult, CustomIdOptions, InteractionCommand, ToCustomId};
use crate::queries::{AutocompleteQueries, ObjectQueries};
use crate::{int_option, CD_CLIENT, CONFIG, LOCALE_XML};
use serenity::all::{AutocompleteChoice, CommandOptionType, CreateCommandOption, ResolvedOption};

pub struct PackageCommand;

pub struct PackageArguments {
    package: i32,
}

impl ToCustomId for PackageArguments {
    const CMD: &'static str = PackageCommand::NAME;

    fn parameters(&self) -> String {
        let PackageArguments { package } = self;
        format!("package={package}")
    }
}

impl<'a> TryFrom<CustomIdOptions<'a>> for PackageArguments {
    type Error = String;

    fn try_from(options: CustomIdOptions<'a>) -> Result<Self, Self::Error> {
        Ok(PackageArguments {
            package: options.parse("package")?,
        })
    }
}

impl<'a> TryFrom<&'a [ResolvedOption<'a>]> for PackageArguments {
    type Error = String;

    fn try_from(options: &'a [ResolvedOption<'a>]) -> Result<Self, Self::Error> {
        Ok(PackageArguments {
            package: int_option!(options, "package"),
        })
    }
}

impl InteractionCommand for PackageCommand {
    const NAME: &'static str = "package";

    const DESCRIPTION: &'static str = "View a package in LEGO Universe!";

    fn options() -> Option<Vec<CreateCommandOption>> {
        Some(vec![CreateCommandOption::new(
            CommandOptionType::Integer,
            "package",
            "An package in LEGO Universe.",
        )
        .required(true)
        .set_autocomplete(true)])
    }

    type Arguments = PackageArguments;

    fn handle_autocomplete(
        autocomplete_option: serenity::model::prelude::AutocompleteOption<'_>,
    ) -> Option<Vec<serenity::all::AutocompleteChoice>> {
        let input = autocomplete_option.value;
        Some(CD_CLIENT.autocomplete_object(input))
    }

    fn run(arguments: Self::Arguments) -> CommandResult {
        let PackageArguments { package: id } = arguments;

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
