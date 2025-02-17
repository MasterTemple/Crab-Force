use crate::commands::buy::BuyArguments;
use crate::commands::earn::EarnArguments;
use crate::commands::reward::RewardArguments;
use crate::commands::smash::{SmashArguments, SmashCommand};
use crate::commands::unpack::UnpackArguments;
use crate::ids::CdClientObjectsId;
use crate::interaction_command::{CommandResult, CustomIdOptions, InteractionCommand, ToCustomId};
use crate::pager::{Pager, START_PAGE};
use crate::queries::{AutocompleteQueries, ObjectQueries};
use crate::{int_option, CD_CLIENT, CONFIG, LOCALE_XML};
use itertools::Itertools;
use serenity::all::{
    AutocompleteChoice, ButtonStyle, CommandOptionType, CreateActionRow, CreateCommandOption,
    CreateSelectMenu, CreateSelectMenuKind, CreateSelectMenuOption, ResolvedOption,
};

pub struct DropCommand;

pub struct DropArguments {
    pub item: i32,
    pub page: usize,
}

impl ToCustomId for DropArguments {
    const CMD: &'static str = DropCommand::NAME;

    fn parameters(&self) -> String {
        let DropArguments { item, page } = self;
        vec![format!("item={item}"), format!("page={page}")].join("&")
    }
}

impl TryFrom<&CustomIdOptions> for DropArguments {
    type Error = String;

    fn try_from(options: &CustomIdOptions) -> Result<Self, Self::Error> {
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

        // ------------ //
        // Create Embed //
        // ------------ //

        let mut embed = CONFIG
            .default_embed()
            .title(format!("{} [{}]", name, id))
            .url(object.explorer_url());

        if let Some(icon_url) = CD_CLIENT.object_icon_url(id) {
            embed = embed.thumbnail(icon_url);
        }

        let entries = object.smashables_chances()?;
        let pager = Pager::new(entries, page, 5);

        for (num, entry) in pager.this_page() {
            let field_name = format!("{}. {:.5}% for {}", num, entry.chance * 100.0, &name);
            let sources: Vec<_> = entry
                .sources
                .iter()
                .map(|source| source.hyperlink_name())
                .collect();
            let value = format!("- {}", sources.join("\n- "));
            embed = embed.field(field_name, value, false);
        }

        // ------------------- //
        // Related Actions Row //
        // ------------------- //

        let page = START_PAGE;
        let item = id;
        let earn_button = EarnArguments { item, page }.to_update_button("Earn");
        let drop_button = DropArguments { item, page }.to_self_button("Drop");
        let unpack_button = UnpackArguments { item, page }.to_update_button("Unpack");
        let reward_button = RewardArguments { item, page }.to_update_button("Reward");
        let buy_button = BuyArguments { item, page }.to_update_button("Buy");

        let item_buttons = CreateActionRow::Buttons(vec![
            earn_button,
            drop_button,
            unpack_button,
            reward_button,
            buy_button,
        ]);

        // ---------------------- //
        // Referenced Objects Row //
        // ---------------------- //

        let smashable_options = pager
            .this_page()
            .into_iter()
            .flat_map(|(_, entry)| {
                entry.sources.into_iter().map(|source| {
                    SmashArguments {
                        smashable: source.0,
                    }
                    .into()
                })
            })
            .take(25)
            .collect();

        let smashable_select = CreateActionRow::SelectMenu(CreateSelectMenu::new(
            SmashCommand::NAME,
            CreateSelectMenuKind::String {
                options: smashable_options,
            },
        ));

        // -------------- //
        // Pagination Row //
        // -------------- //

        let prev_page_button = DropArguments {
            item: id,
            page: pager.prev(),
        }
        .to_update_button(format!("Page {}", pager.prev()))
        .disabled(pager.is_first_page());

        let next_page_button = DropArguments {
            item: id,
            page: pager.next(),
        }
        .to_update_button(format!("Page {}", pager.next()))
        .disabled(pager.is_last_page());
        let page_buttons = CreateActionRow::Buttons(vec![prev_page_button, next_page_button]);

        let components = Some(vec![item_buttons, page_buttons, smashable_select]);

        Ok((embed, components))
    }
}
