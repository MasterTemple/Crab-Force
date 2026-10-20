//! The command framework: argument parsing, custom ids, buttons/menus, and responses
//!
//! Every command takes one main argument (an id resolved from a name, or a number) plus an
//! optional page and tier, so the same [`Target`] can come from a slash command, a button, or a
//! select menu.

use std::fmt::Display;

use serenity::all::{
    ButtonStyle, CommandOptionType, CreateActionRow, CreateButton, CreateCommand,
    CreateCommandOption, CreateInteractionResponseMessage, CreateSelectMenu, CreateSelectMenuKind,
    CreateSelectMenuOption, ResolvedOption, ResolvedValue,
};

use crate::data::GameData;
use crate::embed::{truncate, Embed};
use crate::error::{BotError, BotResult};
use crate::pager::{Pager, START_PAGE};
use crate::search::{sanitize_inline, SearchCategory};

/// Discord allows 5 action rows per message
pub const MAX_ROWS: usize = 5;
/// Discord allows 5 buttons per row
pub const MAX_BUTTONS: usize = 5;
/// Discord allows 25 options per select menu
pub const MAX_SELECT_OPTIONS: usize = 25;
const SELECT_TEXT_LIMIT: usize = 100;
const BUTTON_LABEL_LIMIT: usize = 80;
const CUSTOM_ID_LIMIT: usize = 100;

/// What to do with the message a component was clicked on
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Mode {
    /// send a new message (with a "Back" link to the original)
    Reply,
    /// edit the message the component is on
    Update,
}

impl Mode {
    fn code(self) -> &'static str {
        match self {
            Mode::Reply => "r",
            Mode::Update => "u",
        }
    }
}

/// The arguments every command takes
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Target {
    pub id: i32,
    pub page: usize,
    /// Extra selector, e.g. the rarity tier for `/loottable`
    pub tier: Option<i32>,
}

impl Target {
    pub fn new(id: i32) -> Self {
        Self {
            id,
            page: START_PAGE,
            tier: None,
        }
    }

    pub fn page(mut self, page: usize) -> Self {
        self.page = page;
        self
    }

    pub fn tier(mut self, tier: Option<i32>) -> Self {
        self.tier = tier;
        self
    }
}

/// A parsed component custom id: `{r|u}/{command}/{id}/{page}/{tier|-}[/{discriminator}]`
///
/// The optional discriminator only exists to keep custom ids unique within a message (Discord
/// rejects duplicates, e.g. a "Page 1" button next to a "Drop" button that also opens page 1)
#[derive(Debug, PartialEq, Eq)]
pub struct CustomId {
    pub mode: Mode,
    pub command: String,
    pub target: Target,
}

impl CustomId {
    pub fn encode(mode: Mode, command: &str, target: Target, discriminator: Option<&str>) -> String {
        let tier = target.tier.map_or_else(|| String::from("-"), |t| t.to_string());
        let mut id = format!("{}/{command}/{}/{}/{tier}", mode.code(), target.id, target.page);
        if let Some(extra) = discriminator {
            id.push('/');
            id.push_str(extra);
        }
        truncate(&id, CUSTOM_ID_LIMIT)
    }

    pub fn parse(custom_id: &str) -> BotResult<Self> {
        let stale = || {
            BotError::invalid(
                "This button is from an older version of the bot and no longer works. Please run the command again.",
            )
        };
        let mut parts = custom_id.split('/');
        let mode = match parts.next() {
            Some("r") => Mode::Reply,
            Some("u") => Mode::Update,
            _ => return Err(stale()),
        };
        let command = parts.next().filter(|c| !c.is_empty()).ok_or_else(stale)?.to_string();
        let id = parts.next().and_then(|v| v.parse().ok()).ok_or_else(stale)?;
        let page = parts.next().and_then(|v| v.parse().ok()).unwrap_or(START_PAGE);
        let tier = parts.next().and_then(|v| v.parse().ok());
        Ok(Self {
            mode,
            command,
            target: Target { id, page, tier },
        })
    }
}

/// How the main argument is given
#[derive(Copy, Clone, Debug)]
pub enum OptionKind {
    /// A name with autocomplete (also accepts an id)
    Search(SearchCategory),
    /// A plain number
    Integer { min: i64, max: i64 },
}

/// What a command shows
pub struct Response {
    pub embed: Embed,
    pub components: Vec<CreateActionRow>,
}

pub type CommandResult = BotResult<Response>;

impl Response {
    pub fn new(embed: Embed) -> Self {
        Self {
            embed,
            components: vec![],
        }
    }

    /// Adds a row unless it is empty or the message is already full
    pub fn row(mut self, row: Option<CreateActionRow>) -> Self {
        if let Some(row) = row {
            if self.components.len() < MAX_ROWS {
                self.components.push(row);
            }
        }
        self
    }

    pub fn rows(mut self, rows: impl IntoIterator<Item = CreateActionRow>) -> Self {
        for row in rows {
            self = self.row(Some(row));
        }
        self
    }

    pub fn into_message(self) -> CreateInteractionResponseMessage {
        CreateInteractionResponseMessage::new()
            .embed(self.embed.build())
            .components(self.components)
    }
}

pub trait InteractionCommand {
    const NAME: &'static str;
    const DESCRIPTION: &'static str;
    const OPTION_NAME: &'static str;
    const OPTION_DESCRIPTION: &'static str;
    const OPTION: OptionKind;

    fn run(d: &GameData, target: Target) -> CommandResult;

    fn register() -> CreateCommand {
        let option = match Self::OPTION {
            OptionKind::Search(_) => {
                CreateCommandOption::new(CommandOptionType::String, Self::OPTION_NAME, Self::OPTION_DESCRIPTION)
                    .set_autocomplete(true)
                    .max_length(100)
            }
            OptionKind::Integer { min, max } => {
                CreateCommandOption::new(CommandOptionType::Integer, Self::OPTION_NAME, Self::OPTION_DESCRIPTION)
                    .min_int_value(min as u64)
                    .max_int_value(max as u64)
            }
        };
        CreateCommand::new(Self::NAME)
            .description(Self::DESCRIPTION)
            .add_option(option.required(true))
    }

    /// Reads the main argument from slash command options
    fn target_from_options(d: &GameData, options: &[ResolvedOption]) -> BotResult<Target> {
        let option = options
            .iter()
            .find(|opt| opt.name == Self::OPTION_NAME)
            .ok_or_else(|| BotError::invalid(format!("Please provide `{}`.", Self::OPTION_NAME)))?;
        let id = match (&option.value, Self::OPTION) {
            (ResolvedValue::String(input), OptionKind::Search(category)) => d.search.resolve(category, input)?,
            (ResolvedValue::Integer(n), _) => i32::try_from(*n)
                .map_err(|_| BotError::invalid(format!("`{n}` is too large.")))?,
            (ResolvedValue::String(input), OptionKind::Integer { .. }) => input
                .trim()
                .parse()
                .map_err(|_| BotError::invalid(format!("`{}` is not a number.", sanitize_inline(input))))?,
            (other, _) => {
                return Err(BotError::internal(format!(
                    "unexpected value for `{}`: {other:?}",
                    Self::OPTION_NAME
                )))
            }
        };
        Ok(Target::new(id))
    }

    fn custom_id(mode: Mode, target: Target) -> String {
        CustomId::encode(mode, Self::NAME, target, None)
    }

    /// A button that replaces the current message with this command's output
    fn button(label: impl Into<String>, target: Target) -> CreateButton {
        CreateButton::new(Self::custom_id(Mode::Update, target))
            .label(truncate(&label.into(), BUTTON_LABEL_LIMIT))
            .style(ButtonStyle::Primary)
    }

    /// A related-command button: green and disabled when it is the current command, faded when
    /// `available` is false
    fn nav_button(label: impl Into<String>, id: i32, current: &str, available: bool) -> CreateButton {
        let label = label.into();
        if Self::NAME == current {
            CreateButton::new(format!("self/{}/{id}", Self::NAME))
                .label(truncate(&label, BUTTON_LABEL_LIMIT))
                .style(ButtonStyle::Success)
                .disabled(true)
        } else {
            Self::button(label, Target::new(id)).disabled(!available)
        }
    }

    /// An option for a select menu that opens this command in a new message
    fn select_option(label: impl Display, description: Option<String>, target: Target) -> SelectOption {
        SelectOption {
            label: label.to_string(),
            description,
            value: Self::custom_id(Mode::Reply, target),
        }
    }

    /// Previous/next page buttons, or `None` when there is only one page
    fn page_row<T>(pager: &Pager<T>, target: Target) -> Option<CreateActionRow> {
        if !pager.has_multiple_pages() {
            return None;
        }
        let first = CreateButton::new(CustomId::encode(Mode::Update, Self::NAME, target.page(START_PAGE), Some("first")))
            .label("« First")
            .style(ButtonStyle::Secondary)
            .disabled(pager.is_first_page());
        let prev = CreateButton::new(CustomId::encode(Mode::Update, Self::NAME, target.page(pager.prev()), Some("prev")))
            .label(format!("‹ Page {}", pager.prev()))
            .style(ButtonStyle::Secondary)
            .disabled(pager.is_first_page());
        let next = CreateButton::new(CustomId::encode(Mode::Update, Self::NAME, target.page(pager.next()), Some("next")))
            .label(format!("Page {} ›", pager.next()))
            .style(ButtonStyle::Secondary)
            .disabled(pager.is_last_page());
        let last = CreateButton::new(CustomId::encode(Mode::Update, Self::NAME, target.page(pager.max_page()), Some("last")))
            .label("Last »")
            .style(ButtonStyle::Secondary)
            .disabled(pager.is_last_page());
        Some(CreateActionRow::Buttons(vec![first, prev, next, last]))
    }
}

/// A select menu entry; see [`select_row`]
pub struct SelectOption {
    pub label: String,
    pub description: Option<String>,
    pub value: String,
}

/// A select menu, with duplicate values removed and every Discord limit applied
/// - `menu_id` must be unique within the message
/// - returns `None` when there is nothing to select
pub fn select_row(menu_id: &str, placeholder: &str, options: Vec<SelectOption>) -> Option<CreateActionRow> {
    let mut seen = vec![];
    let options: Vec<CreateSelectMenuOption> = options
        .into_iter()
        .filter(|opt| {
            if seen.contains(&opt.value) {
                false
            } else {
                seen.push(opt.value.clone());
                true
            }
        })
        .take(MAX_SELECT_OPTIONS)
        .map(|opt| {
            let mut option = CreateSelectMenuOption::new(truncate(&opt.label, SELECT_TEXT_LIMIT), opt.value);
            if let Some(desc) = opt.description.filter(|d| !d.trim().is_empty()) {
                option = option.description(truncate(&desc, SELECT_TEXT_LIMIT));
            }
            option
        })
        .collect();
    if options.is_empty() {
        return None;
    }
    Some(CreateActionRow::SelectMenu(
        CreateSelectMenu::new(format!("menu/{menu_id}"), CreateSelectMenuKind::String { options })
            .placeholder(truncate(placeholder, 150)),
    ))
}

/// Puts buttons into rows of at most 5
pub fn button_rows(buttons: Vec<CreateButton>) -> Vec<CreateActionRow> {
    let mut rows = vec![];
    let mut buttons = buttons.into_iter().peekable();
    while buttons.peek().is_some() {
        rows.push(CreateActionRow::Buttons(buttons.by_ref().take(MAX_BUTTONS).collect()));
    }
    rows
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn custom_id_round_trip() {
        let target = Target::new(1727).page(3).tier(Some(2));
        let id = CustomId::encode(Mode::Update, "loottable", target, Some("next"));
        assert_eq!(id, "u/loottable/1727/3/2/next");
        let parsed = CustomId::parse(&id).unwrap();
        assert_eq!(parsed.mode, Mode::Update);
        assert_eq!(parsed.command, "loottable");
        assert_eq!(parsed.target, target);

        let parsed = CustomId::parse("r/drop/5/1/-").unwrap();
        assert_eq!(parsed.target.tier, None);
    }

    #[test]
    fn old_custom_ids_are_rejected_gracefully() {
        assert!(CustomId::parse("u/drop:item=1&page=1").is_err());
        assert!(CustomId::parse("----").is_err());
        assert!(CustomId::parse("").is_err());
    }
}
