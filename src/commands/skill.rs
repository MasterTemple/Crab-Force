use crate::data::GameData;
use crate::embed::Embed;
use crate::ids::{split_camel_case, CdClientSkillBehaviorId, CooldownGroupId, LUExplorer};
use crate::interaction_command::{CommandResult, InteractionCommand, OptionKind, Response, Target};
use crate::loot::format_float;
use crate::search::SearchCategory;

use super::skill_row;

pub struct SkillCommand;

/// Cooldown, cost, and description in a few lines (used by `/skills` and `/cooldowngroup`)
pub fn skill_summary(d: &GameData, skill: CdClientSkillBehaviorId) -> String {
    let mut lines = vec![];
    if let Some(text) = skill.description_text(d) {
        lines.push(text);
    }
    if let Some(sb) = skill.fetch(d) {
        let mut stats = vec![];
        if sb.imaginationcost > 0 {
            stats.push(format!("`{}` imagination", sb.imaginationcost));
        }
        if let Some(cooldown) = sb.cooldown.filter(|&c| c > 0.0) {
            stats.push(format!("`{}s` cooldown", format_float(cooldown)));
        }
        for (value, name) in [
            (sb.armor_bonus_ui, "armor"),
            (sb.life_bonus_ui, "life"),
            (sb.im_bonus_ui, "imagination"),
        ] {
            if let Some(value) = value.filter(|&v| v != 0) {
                stats.push(format!("`+{value}` {name}"));
            }
        }
        if !stats.is_empty() {
            lines.push(stats.join(" • "));
        }
    }
    if lines.is_empty() {
        String::from("No description")
    } else {
        lines.join("\n")
    }
}

impl InteractionCommand for SkillCommand {
    const NAME: &'static str = "skill";
    const DESCRIPTION: &'static str = "View the stats of a skill!";
    const OPTION_NAME: &'static str = "skill";
    const OPTION_DESCRIPTION: &'static str = "A skill in LEGO Universe (name or id).";
    const OPTION: OptionKind = OptionKind::Search(SearchCategory::Skill);

    fn run(d: &GameData, target: Target) -> CommandResult {
        let id = CdClientSkillBehaviorId(target.id);
        let skill = id.try_fetch(d)?;

        let mut embed = Embed::new()
            .title(id.name_id(d))
            .url(id.explorer_url())
            .thumbnail(id.thumbnail(d));

        if let Some(group) = skill.cooldowngroup {
            embed = embed.field("Cooldown Group", CooldownGroupId(group).hyperlink_name(d), true);
        }
        if let Some(cooldown) = skill.cooldown {
            embed = embed.field("Cooldown Time", format!("`{}` seconds", format_float(cooldown)), true);
        }
        embed = embed.field("Imagination Cost", format!("`{}` imagination", skill.imaginationcost), true);
        for (value, name) in [
            (skill.armor_bonus_ui, "Armor Bonus"),
            (skill.life_bonus_ui, "Life Bonus"),
            (skill.im_bonus_ui, "Imagination Bonus"),
            (skill.damage_ui, "Damage"),
        ] {
            if let Some(value) = value.filter(|&v| v != 0) {
                embed = embed.field(name, format!("`{value}`"), true);
            }
        }

        if let Some(desc) = id.description(d) {
            if let Some(remainder) = &desc.remainder {
                embed = embed.field("Description", remainder, false);
            }
            for (header, value) in desc.segments.iter().flatten() {
                embed = embed.field(split_camel_case(header), value, value.chars().count() <= 32);
            }
        }

        let count = id.objects(d).len();
        embed = embed.field("Used By", format!("{count} object{}", if count == 1 { "" } else { "s" }), true);

        Ok(Response::new(embed).row(Some(skill_row(d, id, Self::NAME))))
    }
}
