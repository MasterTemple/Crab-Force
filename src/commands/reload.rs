use serenity::all::{CommandInteraction, CreateCommand, Permissions};

use crate::config::BotConfig;
use crate::error::{BotError, BotResult};

pub const NAME: &str = "reload";

pub fn register() -> CreateCommand {
    CreateCommand::new(NAME)
        .description("Reload cdclient.sqlite and locale.xml (admins only).")
        // hidden from regular members by default; server admins can change this in settings
        .default_member_permissions(Permissions::MANAGE_GUILD)
}

/// Members need one of the configured admin roles; without any configured, Administrator
pub fn check_permission(config: &BotConfig, command: &CommandInteraction) -> BotResult<()> {
    let member = command
        .member
        .as_ref()
        .ok_or_else(|| BotError::Forbidden(String::from("`/reload` can only be used in a server.")))?;
    let allowed = if config.admin_roles.is_empty() {
        member.permissions.is_some_and(|p| p.administrator())
    } else {
        member.roles.iter().any(|role| config.admin_roles.contains(role))
    };
    if allowed {
        Ok(())
    } else {
        Err(BotError::Forbidden(String::from("You do not have permission to use `/reload`.")))
    }
}
