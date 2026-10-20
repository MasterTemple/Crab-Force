use crate::data::GameData;
use crate::ids::{CdClientObjectsId, LUExplorer};
use crate::interaction_command::{CommandResult, InteractionCommand, OptionKind, Response, Target};
use crate::loot::format_number;
use crate::search::SearchCategory;

use super::{item_rows, object_embed};

pub struct BrickCommand;

impl InteractionCommand for BrickCommand {
    const NAME: &'static str = "brick";
    const DESCRIPTION: &'static str = "View the stats of a brick!";
    const OPTION_NAME: &'static str = "brick";
    const OPTION_DESCRIPTION: &'static str = "A brick in LEGO Universe (name or id).";
    const OPTION: OptionKind = OptionKind::Search(SearchCategory::Brick);

    fn run(d: &GameData, target: Target) -> CommandResult {
        let brick = CdClientObjectsId(target.id);
        let object = brick.try_fetch(d)?;
        let ic = brick.item_component(d);

        let embed = object_embed(d, brick)
            .field("Name", brick.req_name(d), true)
            .field("Description", brick.description(d).unwrap_or_else(|| String::from("None")), true)
            .field("Internal Notes", object.internal_notes.clone().unwrap_or_else(|| String::from("None")), true)
            .field(
                "Cost",
                ic.and_then(|ic| ic.base_value)
                    .map_or_else(|| String::from("None"), |v| format!("{} coins", format_number(v.into()))),
                true,
            )
            .field(
                "Stack Size",
                ic.and_then(|ic| ic.stack_size).map_or_else(|| String::from("None"), |s| s.to_string()),
                true,
            )
            .field(
                "Level Requirement",
                brick.level_requirement(d).map_or_else(|| String::from("None"), |l| l.to_string()),
                true,
            );

        Ok(Response::new(embed).rows(item_rows(d, brick, Self::NAME)))
    }
}
