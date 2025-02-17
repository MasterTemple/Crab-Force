use std::collections::BTreeMap;
use std::fmt::Display;
use std::str::FromStr;

use rusqlite::types::FromSql;
use serenity::all::{
    AutocompleteChoice, AutocompleteOption, ButtonStyle, Color, CommandInteraction,
    CommandOptionChoice, CommandOptionType, ComponentInteraction, CreateActionRow, CreateButton,
    CreateCommandOption, CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter,
    CreateInteractionResponseMessage, ResolvedOption, ResolvedValue,
};
use serenity::builder::CreateCommand;
use serenity::json::Value;

use crate::custom::Mutated;
use crate::ids::MsgResult;
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

// pub fn parse_custom_id(custom_id: &str) -> (&str, BTreeMap<&str, &str>) {
//     let colon_idx = custom_id.find(":").unwrap_or(0);
//     let cmd = &custom_id[..colon_idx];
//     let remaining = &custom_id[colon_idx + 1..];
//     let mut options = BTreeMap::new();
//     for seg in remaining.split("&") {
//         if let Some((key, value)) = seg.split_once("=") {
//             options.insert(key, value);
//         }
//     }
//     (cmd, options)
// }

// A leading slash in the custom id means that I should reply with a new message
// pub fn parse_custom_id_options(custom_id: &str) -> CustomIdOptions {
//     let slash_idx = custom_id.find("/").unwrap_or(0);
//     let interaction = &custom_id[..slash_idx];
//     // r = reply, u = update
//     let update_message = interaction == "u";
//
//     let colon_idx = custom_id
//         .find(":")
//         .map(|idx| idx + slash_idx + 1)
//         .unwrap_or(0);
//     let cmd = custom_id[..colon_idx].to_string();
//
//     let remaining = &custom_id[colon_idx + 1..];
//     let mut options = BTreeMap::new();
//     for seg in remaining.split("&") {
//         if let Some((key, value)) = seg.split_once("=") {
//             options.insert(key.to_string(), value.to_string());
//         }
//     }
//     CustomIdOptions {
//         cmd,
//         map: options,
//         update_message,
//     }
// }

#[derive(Debug)]
pub enum CustomIdInteractionType {
    Reply,
    Update,
}

impl FromStr for CustomIdInteractionType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "r" => CustomIdInteractionType::Reply,
            "u" => CustomIdInteractionType::Update,
            other => Err(format!(
                "Could not parse `{other}` into an interaction type"
            ))?,
        })
    }
}

impl Display for CustomIdInteractionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CustomIdInteractionType::Reply => "r",
                CustomIdInteractionType::Update => "u",
            }
        )
    }
}

#[derive(Debug)]
pub struct CustomIdOptions {
    pub interaction: CustomIdInteractionType,
    pub cmd: String,
    pub map: BTreeMap<String, String>,
}

impl CustomIdOptions {
    // pub fn from_custom_id(custom_id: &str) -> CustomIdOptions {
    //     parse_custom_id_options(custom_id)
    // }
    pub fn from_custom_id(custom_id: &str) -> MsgResult<CustomIdOptions> {
        let slash_idx = custom_id
            .find("/")
            .ok_or_else(|| format!("Could not parse interaction type"))?;
        let interaction = &custom_id[..slash_idx];
        let interaction = interaction.parse()?;

        let remaining = &custom_id[slash_idx + 1..];

        let colon_idx = remaining
            .find(":")
            .ok_or_else(|| format!("Could not parse command type"))?;
        let cmd = remaining[..colon_idx].to_string();

        let remaining = &remaining[colon_idx + 1..];
        let mut options = BTreeMap::new();
        for seg in remaining.split("&") {
            if let Some((key, value)) = seg.split_once("=") {
                options.insert(key.to_string(), value.to_string());
            }
        }

        Ok(CustomIdOptions {
            cmd,
            map: options,
            interaction,
        })
    }

    pub fn get(&self, key: &str) -> Result<&str, String> {
        Ok(self
            .map
            .get(key)
            .ok_or_else(|| format!("Parameter Missing: `{key}`"))?)
    }

    pub fn parse<T: FromStr>(&self, key: &str) -> Result<T, String> {
        let value = self.get(key)?;
        value
            .parse::<T>()
            .map_err(|e| format!("Parse of Parameter `{key}` Failed: `{value}`"))
    }

    // pub fn into_custom_id(&self) -> String {
    //     let data = self.map.iter().map(|(key, value)| format!("{key}={value}"));
    //     let data = data.collect::<Vec<_>>().join("&");
    //     let cmd = &self.cmd;
    //     format!("{update_message}/{cmd}:{data}")
    // }
}

pub trait ToCustomId {
    const CMD: &'static str;
    fn to_custom_id(&self, reply: bool) -> String {
        format!(
            "{}/{}:{}",
            if reply { "r" } else { "u" },
            Self::CMD,
            self.parameters()
        )
    }
    fn parameters(&self) -> String;

    /// - The reason for the random custom_id is that Discord won't let me have duplicates
    /// - Duplicates occur when I have Drop button on /drop because of Page 1 button
    fn to_self_button(&self, label: impl Into<String>) -> CreateButton {
        CreateButton::new("----")
            .label(label)
            .style(ButtonStyle::Success)
            .disabled(true)
    }

    fn to_reply_button(&self, label: impl Into<String>) -> CreateButton {
        CreateButton::new(self.to_custom_id(true)).label(label)
    }
    fn to_update_button(&self, label: impl Into<String>) -> CreateButton {
        CreateButton::new(self.to_custom_id(false)).label(label)
    }
}

pub type CommandResult = Result<(CreateEmbed, Option<Vec<CreateActionRow>>), String>;

pub trait InteractionCommand {
    const NAME: &'static str;
    const DESCRIPTION: &'static str;

    /// This is so that it can respond from either a slash command or component interaction
    /// - This type must be parsable from a string (message component id)
    ///   - Format: `{cmd}:{key}={value}&` which is parsed into [`CustomIdOptions`]
    /// - This type must be parsable from a &[ResolvedOption] (slash command)
    type Arguments: for<'a> TryFrom<&'a CustomIdOptions, Error = String>
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

    // fn run(arguments: Self::Arguments) -> (CreateEmbed, Option<Vec<CreateActionRow>>);
    fn run(arguments: Self::Arguments) -> CommandResult;

    fn handle_autocomplete(
        autocomplete_option: AutocompleteOption<'_>,
    ) -> Option<Vec<AutocompleteChoice>> {
        None
    }

    fn handle_slash_command(command: &CommandInteraction) -> CreateInteractionResponseMessage {
        let result = Self::Arguments::try_from(command.data.options().as_slice())
            .and_then(|args| Self::run(args));
        let (embed, components) = result.unwrap_or_else(|msg| CONFIG.error_embed(msg));

        let mut response = CreateInteractionResponseMessage::new().embed(embed);
        if let Some(components) = components {
            response = response.components(components);
        }
        response
    }

    fn handle_component_interaction(
        interaction: &ComponentInteraction,
        options: &CustomIdOptions,
    ) -> CreateInteractionResponseMessage {
        let result = Self::Arguments::try_from(options).and_then(|args| Self::run(args));
        let (embed, components) = result.unwrap_or_else(|msg| CONFIG.error_embed(msg));
        let mut components = components.unwrap_or_else(|| vec![]);

        match options.interaction {
            CustomIdInteractionType::Reply => {
                let back_button = CreateButton::new_link(interaction.message.link()).label("Back");
                let back_component_row = CreateActionRow::Buttons(vec![back_button]);
                components.push(back_component_row);
            }
            CustomIdInteractionType::Update => {}
        };

        CreateInteractionResponseMessage::new()
            .embed(embed)
            .components(components)
    }
}
