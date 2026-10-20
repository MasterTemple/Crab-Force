use crate::data::GameData;
use crate::error::BotError;
use crate::ids::{CdClientObjectsId, LUExplorer};
use crate::interaction_command::{CommandResult, InteractionCommand, OptionKind, Response, Target};
use crate::loot::{format_float, format_number};
use crate::search::SearchCategory;

use super::{enemy_row, object_embed, smash_coins};

pub struct EnemyCommand;

impl InteractionCommand for EnemyCommand {
    const NAME: &'static str = "enemy";
    const DESCRIPTION: &'static str = "View the stats of an enemy!";
    const OPTION_NAME: &'static str = "enemy";
    const OPTION_DESCRIPTION: &'static str = "An enemy in LEGO Universe (name or id).";
    const OPTION: OptionKind = OptionKind::Search(SearchCategory::Enemy);

    fn run(d: &GameData, target: Target) -> CommandResult {
        let enemy = CdClientObjectsId(target.id);
        enemy.try_fetch(d)?;
        let dc = enemy.destructible_component(d).ok_or_else(|| {
            BotError::not_applicable(format!("{} has no health and cannot be smashed.", enemy.hyperlink_name(d)))
        })?;

        let mut embed = object_embed(d, enemy);
        if let Some(description) = enemy.description(d) {
            embed = embed.description(description);
        }
        let stat = |value: Option<i32>| format!("`{}`", format_number(value.unwrap_or(0).into()));
        embed = embed
            .field("Life", stat(dc.life), true)
            .field("Armor", format!("`{}`", format_float(dc.armor.unwrap_or(0.0))), true)
            .field("Imagination", stat(dc.imagination), true);
        if let Some(level) = dc.level.filter(|&l| l > 0) {
            embed = embed.field("Level", format!("`{level}`"), true);
        }
        if let Some(faction) = dc.faction {
            embed = embed.field("Faction", format!("`{faction}`"), true);
        }
        if let Some(coins) = smash_coins(d, dc) {
            embed = embed.field("Coins Dropped", coins, true);
        }
        embed = embed.field("Smashable", if dc.is_smashable { "Yes" } else { "No" }, true);

        Ok(Response::new(embed).row(Some(enemy_row(d, enemy, Self::NAME))))
    }
}
