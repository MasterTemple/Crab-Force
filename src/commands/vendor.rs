use crate::interaction_command::{CommandResult, CustomIdOptions, InteractionCommand, ToCustomId};
use crate::queries::{AutocompleteQueries, ObjectQueries};
use crate::{int_option, CD_CLIENT, CONFIG, LOCALE_XML};
use serenity::all::{AutocompleteChoice, CommandOptionType, CreateCommandOption, ResolvedOption};

pub struct VendorCommand;

pub struct VendorArguments {
    vendor: i32,
}

impl ToCustomId for VendorArguments {
    const CMD: &'static str = VendorCommand::NAME;

    fn parameters(&self) -> String {
        let VendorArguments { vendor } = self;
        format!("vendor={vendor}")
    }
}

impl<'a> TryFrom<CustomIdOptions<'a>> for VendorArguments {
    type Error = String;

    fn try_from(options: CustomIdOptions<'a>) -> Result<Self, Self::Error> {
        Ok(VendorArguments {
            vendor: options.parse("vendor")?,
        })
    }
}

impl<'a> TryFrom<&'a [ResolvedOption<'a>]> for VendorArguments {
    type Error = String;

    fn try_from(options: &'a [ResolvedOption<'a>]) -> Result<Self, Self::Error> {
        Ok(VendorArguments {
            vendor: int_option!(options, "vendor"),
        })
    }
}

impl InteractionCommand for VendorCommand {
    const NAME: &'static str = "vendor";

    const DESCRIPTION: &'static str = "View a vendor in LEGO Universe!";

    fn options() -> Option<Vec<CreateCommandOption>> {
        Some(vec![CreateCommandOption::new(
            CommandOptionType::Integer,
            "vendor",
            "A vendor in LEGO Universe.",
        )
        .required(true)
        .set_autocomplete(true)])
    }

    type Arguments = VendorArguments;

    fn handle_autocomplete(
        autocomplete_option: serenity::model::prelude::AutocompleteOption<'_>,
    ) -> Option<Vec<serenity::all::AutocompleteChoice>> {
        let input = autocomplete_option.value;
        Some(CD_CLIENT.autocomplete_object(input))
    }

    fn run(arguments: Self::Arguments) -> CommandResult {
        let VendorArguments { vendor: id } = arguments;

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
