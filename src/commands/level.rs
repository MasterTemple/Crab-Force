use crate::interaction_command::{CommandResult, CustomIdOptions, InteractionCommand, ToCustomId};
use crate::{int_option, CD_CLIENT, CONFIG};
use serenity::all::{
    CommandOptionType, CreateActionRow, CreateCommandOption, CreateEmbed, ResolvedOption,
};

pub struct LevelCommand;

pub struct LevelArguments {
    level: i32,
}

impl ToCustomId for LevelArguments {
    const CMD: &'static str = LevelCommand::NAME;

    fn parameters(&self) -> String {
        let LevelArguments { level } = self;
        format!("level={level}")
    }
}

impl TryFrom<&CustomIdOptions> for LevelArguments {
    type Error = String;

    fn try_from(options: &CustomIdOptions) -> Result<Self, Self::Error> {
        Ok(LevelArguments {
            level: options.parse("level")?,
        })
    }
}

impl<'a> TryFrom<&'a [ResolvedOption<'a>]> for LevelArguments {
    type Error = String;

    fn try_from(options: &'a [ResolvedOption<'a>]) -> Result<Self, Self::Error> {
        Ok(LevelArguments {
            level: int_option!(options, "level"),
        })
    }
}

impl InteractionCommand for LevelCommand {
    const NAME: &'static str = "level";

    const DESCRIPTION: &'static str = "View stats about a level in LEGO Universe!";

    fn options() -> Option<Vec<CreateCommandOption>> {
        Some(vec![CreateCommandOption::new(
            CommandOptionType::Integer,
            "level",
            "A level in LEGO Universe.",
        )
        .required(true)])
    }

    type Arguments = LevelArguments;

    fn run(arguments: Self::Arguments) -> CommandResult {
        let LevelArguments { level } = arguments;

        let progression = CD_CLIENT
            .level_progression_lookup
            .at_key(&level)
            .ok_or_else(|| format!("Level `{level}` does not exist!"))?;

        let total_experience = progression.required_uscore;
        let experience_for_previous_level = CD_CLIENT
            .level_progression_lookup
            .at_key(&(level - 1))
            .map(|l| l.required_uscore)
            .unwrap_or(0);
        let experience_from_previous_level = total_experience - experience_for_previous_level;

        let min_level = CD_CLIENT.level_progression_lookup.first().unwrap().id;
        let max_level = CD_CLIENT.level_progression_lookup.last().unwrap().id;

        let embed = CONFIG
            .default_embed()
            .title(format!("Level {level}!"))
            .url(CONFIG.explorer_uri("/misc/level-progression"))
            .thumbnail(CONFIG.explorer_uri("/lu-res/ui/ingame/passport_i90.png"))
            .field("Requirements", format!("**For Level {level}:**"), true)
            .field(
                format!("From Level {}", level - 1),
                format!("`{}` Experience", experience_from_previous_level),
                true,
            )
            .field("Total", format!("`{}` Experience", total_experience), true);

        let prev_level_button = LevelArguments { level: level - 1 }
            .to_update_button(format!("Level {}", level - 1))
            .disabled(level - 1 < min_level);
        let next_level_button = LevelArguments { level: level + 1 }
            .to_update_button(format!("Level {}", level + 1))
            .disabled(level + 1 > max_level);

        let components = Some(vec![CreateActionRow::Buttons(vec![
            prev_level_button,
            next_level_button,
        ])]);

        Ok((embed, components))
    }
}
