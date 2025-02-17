use crate::ids::CdClientObjectsId;
use crate::interaction_command::{CommandResult, CustomIdOptions, InteractionCommand, ToCustomId};
use crate::pager::{Pager, START_PAGE};
use crate::queries::{AutocompleteQueries, ObjectQueries};
use crate::{int_option, CD_CLIENT, CONFIG, LOCALE_XML};
use serenity::all::{
    AutocompleteChoice, CommandOptionType, CreateActionRow, CreateCommandOption, CreateSelectMenu,
    CreateSelectMenuKind, CreateSelectMenuOption, ResolvedOption,
};

use super::buy::BuyArguments;
use super::drop::DropArguments;
use super::reward::RewardArguments;
use super::unpack::UnpackArguments;

pub struct EarnCommand;

pub struct EarnArguments {
    pub item: i32,
    pub page: usize,
}

impl ToCustomId for EarnArguments {
    const CMD: &'static str = EarnCommand::NAME;

    fn parameters(&self) -> String {
        let EarnArguments { item, page } = self;
        vec![format!("item={item}"), format!("page={page}")].join("&")
    }
}

impl TryFrom<&CustomIdOptions> for EarnArguments {
    type Error = String;

    fn try_from(options: &CustomIdOptions) -> Result<Self, Self::Error> {
        Ok(EarnArguments {
            item: options.parse("item")?,
            page: options.parse("page")?,
        })
    }
}

impl<'a> TryFrom<&'a [ResolvedOption<'a>]> for EarnArguments {
    type Error = String;

    fn try_from(options: &'a [ResolvedOption<'a>]) -> Result<Self, Self::Error> {
        Ok(EarnArguments {
            item: int_option!(options, "item"),
            page: START_PAGE,
        })
    }
}

impl InteractionCommand for EarnCommand {
    const NAME: &'static str = "earn";

    const DESCRIPTION: &'static str = "View all missions that reward an item!";

    fn options() -> Option<Vec<CreateCommandOption>> {
        Some(vec![CreateCommandOption::new(
            CommandOptionType::Integer,
            "item",
            "An item in LEGO Universe.",
        )
        .required(true)
        .set_autocomplete(true)])
    }

    type Arguments = EarnArguments;

    fn handle_autocomplete(
        autocomplete_option: serenity::model::prelude::AutocompleteOption<'_>,
    ) -> Option<Vec<serenity::all::AutocompleteChoice>> {
        let input = autocomplete_option.value;
        Some(CD_CLIENT.autocomplete_object(input))
    }

    fn run(arguments: Self::Arguments) -> CommandResult {
        let EarnArguments { item: id, page } = arguments;

        // ------------ //
        // Create Embed //
        // ------------ //

        let object = CdClientObjectsId(id);
        let name = object.req_name();

        let mut embed = CONFIG
            .default_embed()
            .title(object.name_id())
            .url(object.explorer_url());

        if let Some(icon_url) = CD_CLIENT.object_icon_url(id) {
            embed = embed.thumbnail(icon_url);
        }

        let entries = object.achievements()?;
        let pager = Pager::new(entries, page, 5);

        // ---------- //
        // Components //
        // ---------- //

        let mut components = vec![];

        // ------------------- //
        // Related Actions Row //
        // ------------------- //

        let page = START_PAGE;
        let item = id;
        let earn_button = EarnArguments { item, page }.to_self_button("Earn");
        let drop_button = DropArguments { item, page }.to_update_button("Drop");
        let unpack_button = UnpackArguments { item, page }.to_update_button("Unpack");
        let reward_button = RewardArguments { item, page }.to_update_button("Reward");
        let buy_button = BuyArguments { item, page }.to_update_button("Buy");

        components.push(CreateActionRow::Buttons(vec![
            earn_button,
            drop_button,
            unpack_button,
            reward_button,
            buy_button,
        ]));

        // ---------------------- //
        // Referenced Objects Row //
        // ---------------------- //

        let options: Vec<CreateSelectMenuOption> = vec![];

        if options.len() > 1 {
            components.push(CreateActionRow::SelectMenu(CreateSelectMenu::new(
                EarnCommand::NAME,
                CreateSelectMenuKind::String { options },
            )));
        }

        // -------------- //
        // Pagination Row //
        // -------------- //

        let prev_page_button = EarnArguments {
            item: id,
            page: pager.prev(),
        }
        .to_update_button(format!("Page {}", pager.prev()))
        .disabled(pager.is_first_page());

        let next_page_button = EarnArguments {
            item: id,
            page: pager.next(),
        }
        .to_update_button(format!("Page {}", pager.next()))
        .disabled(pager.is_last_page());

        if pager.has_multiple_pages() {
            components.push(CreateActionRow::Buttons(vec![
                prev_page_button,
                next_page_button,
            ]));
        }

        Ok((embed, Some(components)))
    }
}
