use crate::data::GameData;
use crate::error::BotError;
use crate::ids::{CdClientObjectsId, LUExplorer};
use crate::interaction_command::{CommandResult, InteractionCommand, OptionKind, Response, Target};
use crate::search::SearchCategory;

use super::{add_loot_matrix, enemy_row, object_embed, smash_coins};

pub struct SmashCommand;

impl InteractionCommand for SmashCommand {
    const NAME: &'static str = "smashable";
    const DESCRIPTION: &'static str = "View everything a smashable or enemy drops!";
    const OPTION_NAME: &'static str = "smashable";
    const OPTION_DESCRIPTION: &'static str = "A smashable or enemy in LEGO Universe (name or id).";
    const OPTION: OptionKind = OptionKind::Search(SearchCategory::Smashable);

    fn run(d: &GameData, target: Target) -> CommandResult {
        let object = CdClientObjectsId(target.id);
        object.try_fetch(d)?;
        let dc = object.destructible_component(d);
        let lmi = dc.and_then(|dc| dc.loot_matrix_index).ok_or_else(|| {
            BotError::not_applicable(format!("{} does not drop anything when smashed.", object.hyperlink_name(d)))
        })?;

        let mut embed = object_embed(d, object);
        if let Some(coins) = dc.and_then(|dc| smash_coins(d, dc)) {
            embed = embed.description(format!("Also drops {coins}."));
        }
        let (embed, menu) = add_loot_matrix(d, embed, lmi, Self::NAME);
        Ok(Response::new(embed).row(menu).row(Some(enemy_row(d, object, Self::NAME))))
    }
}
