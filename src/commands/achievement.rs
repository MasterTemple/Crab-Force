use heck::ToTitleCase;

use crate::cdclient::MissionPreReqType;
use crate::data::GameData;
use crate::ids::{CdClientMissionsId, CdClientObjectsId, LUExplorer};
use crate::interaction_command::{select_row, CommandResult, InteractionCommand, OptionKind, Response, Target};
use crate::loot::format_number;
use crate::search::SearchCategory;

use super::{numbered, object_select_option};

pub struct AchievementCommand;

impl InteractionCommand for AchievementCommand {
    const NAME: &'static str = "achievement";
    const DESCRIPTION: &'static str = "View the stats of an achievement!";
    const OPTION_NAME: &'static str = "achievement";
    const OPTION_DESCRIPTION: &'static str = "An achievement in LEGO Universe (name or id).";
    const OPTION: OptionKind = OptionKind::Search(SearchCategory::Achievement);

    fn run(d: &GameData, target: Target) -> CommandResult {
        mission_response(d, target)
    }
}

/// Shared by `/achievement` and `/mission` (they are the same table)
pub fn mission_response(d: &GameData, target: Target) -> CommandResult {
    let id = CdClientMissionsId(target.id);
    let mission = id.try_fetch(d)?;
    let type_label = if mission.is_mission { "Mission" } else { "Achievement" };

    let mut embed = crate::embed::Embed::new()
        .title(id.name_id(d))
        .url(id.explorer_url())
        .thumbnail(id.thumbnail(d))
        .field(type_label, id.full_path(d), false);

    if let Some(objective) = id.objective(d) {
        embed = embed.field("Objective", objective, false);
    }

    if let Some(prereqs) = &mission.prereq_mission_id {
        let lines: Vec<String> = prereqs
            .iter()
            .enumerate()
            .map(|(idx, req)| match req {
                MissionPreReqType::OneOf(ids) => {
                    let names: Vec<String> = ids.iter().map(|&id| CdClientMissionsId(id).hyperlink_name(d)).collect();
                    numbered(idx + 1, format!("*One of:* {}", names.join(", ")))
                }
                MissionPreReqType::Required(id) => numbered(idx + 1, CdClientMissionsId(*id).hyperlink_name(d)),
            })
            .collect();
        embed = embed.field_lines("Prerequisites", &lines, false);
    }

    let giver = mission.offer_object_id.filter(|&o| o > 0);
    let receiver = mission.target_object_id.filter(|&o| o > 0);
    if let Some(giver) = giver {
        embed = embed.field("Accept From", CdClientObjectsId(giver).hyperlink_name(d), true);
    }
    if let Some(receiver) = receiver {
        embed = embed.field("Return To", CdClientObjectsId(receiver).hyperlink_name(d), true);
    }

    // item rewards
    let rewards = id.rewards(d);
    let mut options = vec![];
    if !rewards.is_empty() {
        let choose_one = mission.is_choice_reward.unwrap_or(false);
        let lines: Vec<String> = rewards
            .iter()
            .enumerate()
            .map(|(idx, r)| {
                let object = CdClientObjectsId(r.item);
                if let Some(option) = object_select_option(d, object, Some(format!("×{}", r.count))) {
                    options.push(option);
                }
                let repeat = r
                    .repeat_count
                    .filter(|&c| c != r.count)
                    .map(|c| format!(" (×{c} when repeated)"))
                    .unwrap_or_default();
                let text = format!("{} ×`{}`{repeat}", object.hyperlink_name(d), r.count);
                if choose_one {
                    format!("- {text}")
                } else {
                    numbered(idx + 1, text)
                }
            })
            .collect();
        let name = if choose_one { "Rewards (choose one)" } else { "Rewards" };
        embed = embed.field_lines(name, &lines, false);
    }

    // emote rewards
    let emotes: Vec<String> = [Some(mission.reward_emote), Some(mission.reward_emote2), mission.reward_emote3, mission.reward_emote4]
        .into_iter()
        .flatten()
        .filter(|&e| e > 0)
        .enumerate()
        .map(|(idx, emote)| {
            let name = d
                .cdclient
                .emotes
                .at_key(&emote)
                .and_then(|e| e.animation_name.as_ref().map(|n| n.to_title_case()))
                .unwrap_or_else(|| format!("Emote {emote}"));
            numbered(idx + 1, format!("{name} `[{emote}]`"))
        })
        .collect();
    if !emotes.is_empty() {
        embed = embed.field_lines("Emotes", &emotes, true);
    }

    // stat rewards
    let stat = |value: i64, unit: &str| format!("`{}` {unit}", format_number(value));
    if mission.lego_score > 0 {
        embed = embed.field("Universe Score", stat(mission.lego_score.into(), "U-Score"), true);
    }
    if let Some(coins) = mission.reward_currency.filter(|&c| c > 0) {
        let repeat = mission
            .reward_currency_repeatable
            .filter(|&c| mission.repeatable && c != coins)
            .map(|c| format!(" ({} when repeated)", format_number(c)))
            .unwrap_or_default();
        embed = embed.field("Coins", stat(coins, "Coins") + &repeat, true);
    }
    if let Some(rep) = mission.reward_reputation.filter(|&r| r > 0) {
        embed = embed.field("Reputation", stat(rep, "Reputation"), true);
    }
    for (value, name, unit) in [
        (mission.reward_maxinventory, "Backpack", "Slots"),
        (mission.reward_maxmodel.unwrap_or(0), "Model Slots", "Slots"),
        (mission.reward_bankinventory.unwrap_or(0), "Vault", "Slots"),
        (mission.reward_maxhealth, "Life", "Life"),
        (mission.reward_maximagination, "Imagination", "Imagination"),
    ] {
        if value > 0 {
            embed = embed.field(name, format!("+{}", stat(value.into(), unit)), true);
        }
    }
    if mission.repeatable {
        embed = embed.field("Repeatable", "Yes", true);
    }

    Ok(Response::new(embed).row(select_row("rewards", "Open a reward…", options)))
}
