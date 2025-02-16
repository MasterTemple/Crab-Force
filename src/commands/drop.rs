use crate::ids::CdClientObjectsId;
use crate::interaction_command::{CommandResult, CustomIdOptions, InteractionCommand, ToCustomId};
use crate::queries::{AutocompleteQueries, ObjectQueries};
use crate::{int_option, CD_CLIENT, CONFIG, LOCALE_XML};
use serenity::all::{AutocompleteChoice, CommandOptionType, CreateCommandOption, ResolvedOption};

pub struct DropCommand;

pub struct DropArguments {
    item: i32,
}

impl ToCustomId for DropArguments {
    const CMD: &'static str = DropCommand::NAME;

    fn parameters(&self) -> String {
        let DropArguments { item } = self;
        format!("item={item}")
    }
}

impl<'a> TryFrom<CustomIdOptions<'a>> for DropArguments {
    type Error = String;

    fn try_from(options: CustomIdOptions<'a>) -> Result<Self, Self::Error> {
        Ok(DropArguments {
            item: options.parse("item")?,
        })
    }
}

impl<'a> TryFrom<&'a [ResolvedOption<'a>]> for DropArguments {
    type Error = String;

    fn try_from(options: &'a [ResolvedOption<'a>]) -> Result<Self, Self::Error> {
        Ok(DropArguments {
            item: int_option!(options, "item"),
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
        let DropArguments { item: id } = arguments;

        let object = CdClientObjectsId(id);
        let name = object.req_name();

        let mut embed = CONFIG
            .default_embed()
            .title(format!("{} [{}]", name, id))
            .url(object.explorer_url());

        if let Some(icon_url) = CD_CLIENT.object_icon_url(id) {
            embed = embed.thumbnail(icon_url);
        }

        let page = 1;
        let page_size = 15;

        let entries = dbg!(object.smashables_chances())?;
        let start = (page - 1) * page_size;
        _ = dbg!(&start);
        let end = std::cmp::min(start + page_size, entries.len());
        _ = dbg!(&end);
        // -----------------------------------------------------------------------------------------
        // ! check if they have gone beyond the page and then calculate last page and put them there
        // -----------------------------------------------------------------------------------------
        let paged_entries = &entries[start..end];
        _ = dbg!(&paged_entries);
        for (idx, entry) in paged_entries.into_iter().enumerate() {
            // for (idx, entry) in entries
            //     .into_iter()
            //     .enumerate()
            //     .skip(start)
            //     .take(end - start)
            // {

            let num = idx + 1;
            let field_name = format!("{}. {:.4}% for {}", num, entry.chance * 100.0, &name);
            _ = dbg!(&field_name);
            let sources: Vec<_> = entry
                .sources
                .iter()
                .map(|source| source.hyperlink_name())
                .collect();
            _ = dbg!(&sources);
            let value = format!("*From* {}", sources.join(", "));
            _ = dbg!(&value);
            embed = embed.field(field_name, value, false);
        }

        Ok((embed, None))
    }
}
