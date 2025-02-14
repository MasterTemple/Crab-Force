use std::collections::BTreeMap;
use std::str::FromStr;

use rusqlite::types::FromSql;
use serenity::all::{
    AutocompleteChoice, AutocompleteOption, Color, CommandInteraction, CommandOptionChoice,
    CommandOptionType, ComponentInteraction, CreateActionRow, CreateButton, CreateCommandOption,
    CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, CreateInteractionResponseMessage,
    ResolvedOption, ResolvedValue,
};
use serenity::builder::CreateCommand;
use serenity::json::Value;

use crate::custom::Mutated;
use crate::{CD_CLIENT, CONFIG};

/// **IMPORTANT: This should only be used on required fields**
#[macro_export]
macro_rules! string_option {
    ($options:ident, $name:literal) => {{
        let field = &$options
            .iter()
            .find(|opt| opt.name == $name)
            .as_ref()
            .ok_or_else(|| format!("Expected required field: `{}`", $name))?
            .value;
        let serenity::all::ResolvedValue::String(field) = field else {
            Err(format!(
                "Incorrect type for field `{}`\nGiven:\n```rust\n{:#?}\n```",
                $name, field
            ))?
        };
        field.clone()
    }};
}

/// **IMPORTANT: This should only be used on required fields**
#[macro_export]
macro_rules! float_option {
    ($options:ident, $name:literal) => {{
        let field = &$options
            .iter()
            .find(|opt| opt.name == $name)
            .as_ref()
            .ok_or_else(|| format!("Expected required field: `{}`", $name))?
            .value;
        let serenity::all::ResolvedValue::Number(field) = field else {
            Err(format!(
                "Incorrect type for field `{}`\nGiven:\n```rust\n{:#?}\n```",
                $name, field
            ))?
        };
        *field
    }};
}

/// **IMPORTANT: This should only be used on required fields**
#[macro_export]
macro_rules! int_option {
    ($options:ident, $name:literal) => {{
        let field = &$options
            .iter()
            .find(|opt| opt.name == $name)
            .as_ref()
            .ok_or_else(|| format!("Expected required field: `{}`", $name))?
            .value;
        let serenity::all::ResolvedValue::Integer(field) = field else {
            Err(format!(
                "Incorrect type for field `{}`\nGiven:\n```rust\n{:#?}\n```",
                $name, field
            ))?
        };
        *field as i32
    }};
}

pub fn parse_custom_id(custom_id: &str) -> (&str, BTreeMap<&str, &str>) {
    let colon_idx = custom_id.find(":").unwrap_or(0);
    let cmd = &custom_id[..colon_idx];
    let remaining = &custom_id[colon_idx + 1..];
    let mut options = BTreeMap::new();
    for seg in remaining.split("&") {
        if let Some((key, value)) = seg.split_once("=") {
            options.insert(key, value);
        }
    }
    (cmd, options)
}

pub fn parse_custom_id_options<'a>(custom_id: &'a str) -> CustomIdOptions<'a> {
    let colon_idx = custom_id.find(":").unwrap_or(0);
    let remaining = &custom_id[colon_idx + 1..];
    let mut options = BTreeMap::new();
    for seg in remaining.split("&") {
        if let Some((key, value)) = seg.split_once("=") {
            options.insert(key, value);
        }
    }
    CustomIdOptions(options)
}

pub struct CustomIdOptions<'a>(BTreeMap<&'a str, &'a str>);
impl<'a> CustomIdOptions<'a> {
    pub fn from_custom_id(custom_id: &'a str) -> CustomIdOptions<'a> {
        parse_custom_id_options(custom_id)
    }

    pub fn get(&self, key: &str) -> Result<&str, String> {
        Ok(self
            .0
            .get(key)
            .ok_or_else(|| format!("Parameter Missing: `{key}`"))?)
    }

    pub fn parse<T: FromStr>(&self, key: &str) -> Result<T, String> {
        let value = self.get(key)?;
        value
            .parse::<T>()
            .map_err(|e| format!("Parse of Parameter `{key}` Failed: `{value}`"))
    }

    pub fn into_custom_id(&self, cmd: &str) -> String {
        let data = self.0.iter().map(|(key, value)| format!("{key}={value}"));
        let data = data.collect::<Vec<_>>().join("&");
        format!("{cmd}:{data}")
    }
}

pub trait ToCustomId {
    const CMD: &'static str;
    fn to_custom_id(&self) -> String {
        format!("{}:{}", Self::CMD, self.parameters())
    }
    fn parameters(&self) -> String;
    fn to_button(&self, label: impl Into<String>) -> CreateButton {
        CreateButton::new(self.to_custom_id()).label(label)
    }
}

pub trait InteractionCommand {
    const NAME: &'static str;
    const DESCRIPTION: &'static str;

    /// This is so that it can respond from either a slash command or component interaction
    /// - This type must be parsable from a string (message component id)
    ///   - Format: `{cmd}:{key}={value}&` which is parsed into [`CustomIdOptions`]
    /// - This type must be parsable from a &[ResolvedOption] (slash command)
    type Arguments: for<'a> TryFrom<CustomIdOptions<'a>, Error = String>
        + for<'a> TryFrom<&'a [ResolvedOption<'a>], Error = String>
        + ToCustomId;
    // I'm holding off on this for now because it requires me to create strings :(
    // + for<'a> Into<CustomIdOptions<'a>>;

    fn options() -> Option<Vec<CreateCommandOption>> {
        None
    }

    fn register() -> CreateCommand {
        let mut cmd = CreateCommand::new(Self::NAME).description(Self::DESCRIPTION);
        if let Some(options) = Self::options() {
            for option in options.clone() {
                cmd = cmd.add_option(option)
            }
        }
        cmd
    }

    fn run(arguments: Self::Arguments) -> (CreateEmbed, Option<Vec<CreateActionRow>>);

    fn handle_autocomplete(
        autocomplete_option: AutocompleteOption<'_>,
    ) -> Option<Vec<AutocompleteChoice>> {
        None
    }

    fn handle_slash_command(command: &CommandInteraction) -> CreateInteractionResponseMessage {
        let (embed, components) = match Self::Arguments::try_from(command.data.options().as_slice())
        {
            Ok(args) => Self::run(args),
            Err(msg) => CONFIG.error_msg(msg),
        };

        let mut response = CreateInteractionResponseMessage::new().embed(embed);
        if let Some(components) = components {
            response = response.components(components);
        }
        response
    }

    fn handle_component_interaction(
        interaction: &ComponentInteraction,
    ) -> CreateInteractionResponseMessage {
        let options = CustomIdOptions::from_custom_id(interaction.data.custom_id.as_str());

        let (embed, components) = match Self::Arguments::try_from(options) {
            Ok(args) => Self::run(args),
            Err(msg) => CONFIG.error_msg(msg),
        };

        let back_button = CreateButton::new_link(interaction.message.link()).label("Back");
        let back_component_row = CreateActionRow::Buttons(vec![back_button]);
        let components = components
            .unwrap_or_else(|| vec![])
            .mutated(|components| components.push(back_component_row));

        CreateInteractionResponseMessage::new()
            .embed(embed)
            .components(components)
    }
}
