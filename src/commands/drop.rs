use crate::ids::CdClientObjectsId;
use crate::interaction_command::{CommandResult, CustomIdOptions, InteractionCommand, ToCustomId};
use crate::pager::{Pager, START_PAGE};
use crate::queries::{AutocompleteQueries, ObjectQueries};
use crate::{int_option, CD_CLIENT, CONFIG, LOCALE_XML};
use itertools::Itertools;
use serenity::all::{
    AutocompleteChoice, CommandOptionType, CreateActionRow, CreateCommandOption, ResolvedOption,
};

pub struct DropCommand;

pub struct DropArguments {
    item: i32,
    page: usize,
}

impl ToCustomId for DropArguments {
    const CMD: &'static str = DropCommand::NAME;

    fn parameters(&self) -> String {
        let DropArguments { item, page } = self;
        vec![format!("item={item}"), format!("page={page}")].join("&")
    }
}

impl<'a> TryFrom<CustomIdOptions<'a>> for DropArguments {
    type Error = String;

    fn try_from(options: CustomIdOptions<'a>) -> Result<Self, Self::Error> {
        Ok(DropArguments {
            item: options.parse("item")?,
            page: options.parse("page")?,
        })
    }
}

impl<'a> TryFrom<&'a [ResolvedOption<'a>]> for DropArguments {
    type Error = String;

    fn try_from(options: &'a [ResolvedOption<'a>]) -> Result<Self, Self::Error> {
        Ok(DropArguments {
            item: int_option!(options, "item"),
            page: START_PAGE,
        })
    }
}

impl InteractionCommand for DropCommand {
    const NAME: &'static str = "drop";

    const DESCRIPTION: &'static str = "View all smashables that drop an item!";

    fn options() -> Option<Vec<CreateCommandOption>> {
        Some(vec![CreateCommandOption::new(
            CommandOptionType::Integer,
            "item",
            "An item in LEGO Universe.",
        )
        .required(true)
        .set_autocomplete(true)])
    }

    type Arguments = DropArguments;

    fn handle_autocomplete(
        autocomplete_option: serenity::model::prelude::AutocompleteOption<'_>,
    ) -> Option<Vec<serenity::all::AutocompleteChoice>> {
        let input = autocomplete_option.value;
        Some(CD_CLIENT.autocomplete_object(input))
    }

    fn run(arguments: Self::Arguments) -> CommandResult {
        let DropArguments { item: id, page } = arguments;

        let object = CdClientObjectsId(id);
        let name = object.req_name();

        let mut embed = CONFIG
            .default_embed()
            .title(format!("{} [{}]", name, id))
            .url(object.explorer_url());

        if let Some(icon_url) = CD_CLIENT.object_icon_url(id) {
            embed = embed.thumbnail(icon_url);
        }

        let entries = object.smashables_chances()?;
        let pager = Pager::new(entries, page, 5);
        dbg!(&pager);
        dbg!(&pager.this_page());

        for (num, entry) in pager.this_page() {
            let field_name = format!("{}. {:.5}% for {}", num, entry.chance * 100.0, &name);
            let sources: Vec<_> = entry
                .sources
                .iter()
                .map(|source| source.hyperlink_name())
                .collect();
            // let value = format!("**From** {}", sources.join("**,** "));
            let value = format!("- {}", sources.join("\n- "));
            embed = embed.field(field_name, value, false);
        }

        let prev_page_button = DropArguments {
            item: id,
            page: pager.prev(),
        }
        .to_button(format!("Page {}", pager.prev()))
        .disabled(pager.is_first_page());

        let next_page_button = DropArguments {
            item: id,
            page: pager.next(),
        }
        .to_button(format!("Page {}", pager.next()))
        .disabled(pager.is_last_page());

        let components = Some(vec![CreateActionRow::Buttons(vec![
            prev_page_button,
            next_page_button,
        ])]);

        Ok((embed, components))
    }
}
