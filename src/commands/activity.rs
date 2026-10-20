use crate::data::GameData;
use crate::embed::Embed;
use crate::ids::{CdClientActivityId, CdClientObjectsId, LUExplorer};
use crate::interaction_command::{select_row, CommandResult, CustomId, InteractionCommand, Mode, OptionKind, Response, SelectOption, Target};
use crate::pager::Pager;
use crate::search::SearchCategory;

use super::{add_loot_matrix, coin_range};

pub struct ActivityCommand;

/// Reward rows in display order (one page each)
fn sorted_rewards(d: &GameData, activity: CdClientActivityId) -> Vec<crate::cdclient::CdClientActivityRewards> {
    let mut rewards = activity.rewards(d).to_vec();
    rewards.sort_by_key(|r| (r.activity_reward_index, r.activity_rating, r.challenge_rating));
    rewards
}

/// The page of `/activity` showing the reward that uses `loot_matrix`, and its description
pub fn activity_page(d: &GameData, activity: CdClientActivityId, loot_matrix: i32) -> (usize, Option<String>) {
    sorted_rewards(d, activity)
        .iter()
        .enumerate()
        .find(|(_, r)| r.loot_matrix_index == Some(loot_matrix))
        .map_or((1, None), |(idx, r)| {
            (idx + 1, Some(r.description.clone()).filter(|desc| !desc.is_empty()))
        })
}

impl InteractionCommand for ActivityCommand {
    const NAME: &'static str = "activity";
    const DESCRIPTION: &'static str = "View all rewards given from an activity!";
    const OPTION_NAME: &'static str = "activity";
    const OPTION_DESCRIPTION: &'static str = "An activity in LEGO Universe (name or id).";
    const OPTION: OptionKind = OptionKind::Search(SearchCategory::Activity);

    /// One page per reward tier (e.g. per survival time or race placement) so embeds stay small
    fn run(d: &GameData, target: Target) -> CommandResult {
        let activity = CdClientActivityId(target.id);
        let rewards = sorted_rewards(d, activity);
        if rewards.is_empty() {
            return Err(activity.not_found(d));
        }
        let pager = Pager::new(rewards.clone(), target.page, 1);
        let Some((_, reward)) = pager.this_page().next() else {
            return Err(activity.not_found(d));
        };

        let mut embed = Embed::new()
            .title(format!("{}{}", activity.name_id(d), pager.title_suffix()))
            .url(activity.explorer_url())
            .field("Reward", if reward.description.is_empty() { format!("Reward {}", reward.activity_reward_index) } else { reward.description.clone() }, false);

        if let Some(info) = d.cdclient.activities.at_key(&activity.0) {
            if let (Some(min), Some(max)) = (info.min_team_size, info.max_team_size) {
                if max > 0 {
                    embed = embed.field("Team Size", if min == max { min.to_string() } else { format!("{min}–{max}") }, true);
                }
            }
            if let (Some(lot), Some(count)) = (info.optional_cost_lot.filter(|&l| l > 0), info.optional_cost_count) {
                embed = embed.field("Cost", format!("{count} × {}", CdClientObjectsId(lot).hyperlink_name(d)), true);
            }
        }
        if let Some(coins) = coin_range(d, reward.currency_index, Some(reward.challenge_rating.max(1))) {
            embed = embed.field("Coins", coins, true);
        }

        let mut menu = None;
        match reward.loot_matrix_index {
            Some(lmi) => {
                let (with_loot, loot_menu) = add_loot_matrix(d, embed, lmi, Self::NAME);
                embed = with_loot;
                menu = loot_menu;
            }
            None => embed = embed.field("Items", "None", false),
        }

        // jump straight to another reward tier
        let tiers: Vec<SelectOption> = rewards
            .iter()
            .enumerate()
            .map(|(idx, r)| SelectOption {
                label: if r.description.is_empty() { format!("Reward {}", idx + 1) } else { r.description.clone() },
                description: Some(format!("Page {}", idx + 1)),
                value: CustomId::encode(Mode::Update, Self::NAME, Target::new(activity.0).page(idx + 1), Some("jump")),
            })
            .collect();
        let tier_menu = (tiers.len() > 1).then(|| select_row("tiers", "Jump to a reward…", tiers)).flatten();

        Ok(Response::new(embed)
            .row(tier_menu)
            .row(menu)
            .row(Self::page_row(&pager, target)))
    }
}
