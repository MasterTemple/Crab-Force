//! Bot configuration, read once from the environment (and `.env`) at startup

use std::{env, path::PathBuf, sync::OnceLock};

use serenity::all::{Colour, RoleId};

use crate::error::ConfigError;

#[derive(Debug, Clone)]
pub struct BotConfig {
    pub token: String,
    /// When set, commands are registered to this guild only (updates instantly);
    /// otherwise they are registered globally
    pub guild_id: Option<u64>,
    pub cdclient_path: PathBuf,
    pub locale_path: PathBuf,
    pub locale: String,
    pub explorer_url: String,
    pub explorer_res_url: String,
    pub color: Colour,
    pub author_name: Option<String>,
    pub author_icon: Option<String>,
    pub author_url: Option<String>,
    pub footer_text: Option<String>,
    pub footer_icon: Option<String>,
    /// Only show/search objects with `HQ_valid = 1`
    pub hq_valid_only: bool,
    pub decimal_places: usize,
    /// Roles allowed to use `/reload`; when empty, only members with Administrator may
    pub admin_roles: Vec<RoleId>,
}

static CONFIG: OnceLock<BotConfig> = OnceLock::new();

/// The global config
///
/// Falls back to defaults (with no token/paths) if [`init`] was never called, which only
/// happens in tests
pub fn config() -> &'static BotConfig {
    CONFIG.get_or_init(BotConfig::defaults)
}

pub fn init(config: BotConfig) {
    // only the first call wins; later calls (there should be none) are ignored
    let _ = CONFIG.set(config);
}

fn var(name: &'static str) -> Option<String> {
    env::var(name)
        .ok()
        .map(|v| v.trim().trim_matches(['"', '\'']).trim().to_string())
        .filter(|v| !v.is_empty())
}

fn required(name: &'static str) -> Result<String, ConfigError> {
    var(name).ok_or(ConfigError::Missing(name))
}

/// Expands a leading `~/` and makes sure the path exists
fn path_var(name: &'static str) -> Result<PathBuf, ConfigError> {
    let raw = required(name)?;
    let path = match raw.strip_prefix("~/") {
        Some(rest) => env::var("HOME")
            .map(|home| PathBuf::from(home).join(rest))
            .unwrap_or_else(|_| PathBuf::from(&raw)),
        None => PathBuf::from(&raw),
    };
    if !path.is_file() {
        return Err(ConfigError::Invalid {
            name,
            value: raw,
            reason: String::from("no file exists at this path"),
        });
    }
    Ok(path)
}

fn bool_var(name: &'static str, default: bool) -> Result<bool, ConfigError> {
    match var(name) {
        None => Ok(default),
        Some(value) => match value.to_lowercase().as_str() {
            "true" | "1" | "yes" | "on" => Ok(true),
            "false" | "0" | "no" | "off" => Ok(false),
            _ => Err(ConfigError::Invalid {
                name,
                value,
                reason: String::from("expected `true` or `false`"),
            }),
        },
    }
}

fn parse_color(name: &'static str, value: &str) -> Result<Colour, ConfigError> {
    let hex = value.trim_start_matches('#').trim_start_matches("0x");
    match u32::from_str_radix(hex, 16) {
        Ok(rgb) if hex.len() == 6 => Ok(Colour::new(rgb)),
        _ => Err(ConfigError::Invalid {
            name,
            value: value.to_string(),
            reason: String::from("expected a hex color such as `#1a87e8`"),
        }),
    }
}

fn ensure_trailing_slash(mut url: String) -> String {
    if !url.ends_with('/') {
        url.push('/');
    }
    url
}

fn join_paths(front: &str, back: &str) -> String {
    let front = front.strip_suffix('/').unwrap_or(front);
    let back = back.strip_prefix('/').unwrap_or(back);
    format!("{front}/{back}")
}

impl BotConfig {
    fn defaults() -> Self {
        BotConfig {
            token: String::new(),
            guild_id: None,
            cdclient_path: PathBuf::new(),
            locale_path: PathBuf::new(),
            locale: String::from("en_US"),
            explorer_url: String::from("https://explorer.lu/"),
            explorer_res_url: String::from("https://explorer.lu/lu-res/"),
            color: Colour::new(0x1a87e8),
            author_name: Some(String::from("Crab Force")),
            author_icon: None,
            author_url: Some(String::from("https://github.com/MasterTemple/Crab-Force")),
            footer_text: Some(String::from("LEGO® is a trademark of the LEGO Group which does not sponsor, authorize, or endorse this bot. The data and assets are presented purely for informational purposes.")),
            footer_icon: None,
            hq_valid_only: false,
            decimal_places: 4,
            admin_roles: vec![],
        }
    }

    /// Reads the configuration from environment variables, reporting every problem precisely
    pub fn from_env() -> Result<Self, ConfigError> {
        let defaults = Self::defaults();

        let guild_id = match var("GUILD_ID") {
            None => None,
            Some(value) => Some(value.parse().map_err(|_| ConfigError::Invalid {
                name: "GUILD_ID",
                value,
                reason: String::from("expected a numeric Discord server id"),
            })?),
        };

        let explorer_url = ensure_trailing_slash(var("EXPLORER_URL").unwrap_or(defaults.explorer_url));
        let explorer_res_url = ensure_trailing_slash(
            var("EXPLORER_RES_URL").unwrap_or_else(|| join_paths(&explorer_url, "lu-res/")),
        );

        let color = match var("BOT_COLOR") {
            Some(value) => parse_color("BOT_COLOR", &value)?,
            None => defaults.color,
        };

        let decimal_places = match var("DECIMAL_PLACES") {
            None => defaults.decimal_places,
            Some(value) => match value.parse::<usize>() {
                Ok(n) if n <= 10 => n,
                _ => {
                    return Err(ConfigError::Invalid {
                        name: "DECIMAL_PLACES",
                        value,
                        reason: String::from("expected a whole number from 0 to 10"),
                    })
                }
            },
        };

        let admin_roles = match var("ADMIN_ROLE_IDS") {
            None => vec![],
            Some(value) => value
                .split(|c: char| !c.is_ascii_digit())
                .filter(|id| !id.is_empty())
                .map(|id| {
                    id.parse::<u64>()
                        .ok()
                        .filter(|&id| id != 0)
                        .map(RoleId::new)
                        .ok_or_else(|| ConfigError::Invalid {
                            name: "ADMIN_ROLE_IDS",
                            value: value.clone(),
                            reason: format!("`{id}` is not a valid role id"),
                        })
                })
                .collect::<Result<_, _>>()?,
        };

        Ok(BotConfig {
            // only required to connect; `check`/`preview` work without it
            token: var("DISCORD_TOKEN").unwrap_or_default(),
            guild_id,
            cdclient_path: path_var("CDCLIENT_PATH")?,
            locale_path: path_var("LOCALE_PATH")?,
            locale: var("LOCALE").unwrap_or(defaults.locale),
            explorer_url,
            explorer_res_url,
            color,
            author_name: var("AUTHOR_NAME").or(defaults.author_name),
            author_icon: var("AUTHOR_ICON").or(defaults.author_icon),
            author_url: var("AUTHOR_URL").or(defaults.author_url),
            footer_text: var("FOOTER_TEXT").or(defaults.footer_text),
            footer_icon: var("FOOTER_ICON").or(defaults.footer_icon),
            hq_valid_only: bool_var("HQ_VALID_ONLY", defaults.hq_valid_only)?,
            decimal_places,
            admin_roles,
        })
    }

    pub fn explorer_uri(&self, path: impl AsRef<str>) -> String {
        join_paths(&self.explorer_url, path.as_ref())
    }

    /// Converts a client asset path (e.g. `..\..\textures\ui\x.dds`) to a hosted png url
    pub fn explorer_res_uri(&self, asset: &str) -> String {
        let asset = asset
            .replace('\\', "/")
            .replace("../", "")
            .replace("./", "")
            .to_lowercase();
        let asset = asset.trim_start_matches('/');
        let asset = asset.strip_prefix("res/").unwrap_or(asset);
        let asset = match asset.strip_suffix(".dds") {
            Some(stem) => format!("{stem}.png"),
            None => asset.to_string(),
        };
        join_paths(&self.explorer_res_url, &asset.replace(' ', "%20"))
    }

    /// Formats a probability (0..=1) as a percentage
    pub fn percent(&self, chance: f64) -> String {
        format!("{:.*}%", self.decimal_places, chance * 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn res_uri_is_normalized() {
        let config = BotConfig::defaults();
        assert_eq!(
            config.explorer_res_uri(r"..\..\textures\ui\inventory\Models\Amb Crab.DDS"),
            "https://explorer.lu/lu-res/textures/ui/inventory/models/amb%20crab.png"
        );
    }

    #[test]
    fn colors() {
        assert!(parse_color("X", "#1a87e8").is_ok());
        assert!(parse_color("X", "1a87e8").is_ok());
        assert!(parse_color("X", "#fff").is_err());
        assert!(parse_color("X", "blue").is_err());
    }
}
