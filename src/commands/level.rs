use serenity::all::CreateActionRow;

use crate::data::GameData;
use crate::error::BotError;
use crate::interaction_command::{CommandResult, InteractionCommand, OptionKind, Response, Target};
use crate::loot::format_number;

pub struct LevelCommand;

impl InteractionCommand for LevelCommand {
    const NAME: &'static str = "level";
    const DESCRIPTION: &'static str = "View stats about a level in LEGO Universe!";
    const OPTION_NAME: &'static str = "level";
    const OPTION_DESCRIPTION: &'static str = "A level in LEGO Universe.";
    const OPTION: OptionKind = OptionKind::Integer { min: 1, max: 1000 };

    fn run(d: &GameData, target: Target) -> CommandResult {
        let levels = &d.cdclient.level_progression_lookup;
        let (Some(first), Some(last)) = (levels.first(), levels.last()) else {
            return Err(BotError::not_applicable("No level data was found in the cdclient."));
        };
        let (min_level, max_level) = (first.id, last.id);
        let level = target.id;
        let progression = levels.at_key(&level).ok_or_else(|| {
            BotError::not_found(format!("Level `{level}` does not exist. Levels go from {min_level} to {max_level}."))
        })?;

        let total = progression.required_uscore;
        let previous = levels.at_key(&(level - 1)).map_or(0, |l| l.required_uscore);
        let next = levels.at_key(&(level + 1)).map(|l| l.required_uscore);

        let config = crate::config::config();
        let mut embed = crate::embed::Embed::new()
            .title(format!("Level {level}"))
            .url(config.explorer_uri("misc/level-progression"))
            .thumbnail(Some(config.explorer_res_uri("ui/ingame/passport_i90.png")))
            .field("Total U-Score", format!("`{}`", format_number(total.into())), true);
        if level > min_level {
            embed = embed.field(
                format!("From Level {}", level - 1),
                format!("`{}`", format_number((total - previous).into())),
                true,
            );
        }
        match next {
            Some(next) => {
                embed = embed.field(
                    format!("To Level {}", level + 1),
                    format!("`{}`", format_number((next - total).into())),
                    true,
                )
            }
            None => embed = embed.field("Max Level", "This is the highest level!", true),
        }

        let row = CreateActionRow::Buttons(vec![
            Self::button("‹ Previous Level", Target::new(level - 1)).disabled(level <= min_level),
            Self::button("Next Level ›", Target::new(level + 1)).disabled(level >= max_level),
        ]);
        Ok(Response::new(embed).row(Some(row)))
    }
}
