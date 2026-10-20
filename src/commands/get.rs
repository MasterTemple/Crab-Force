use crate::data::GameData;
use crate::ids::CdClientObjectsId;
use crate::interaction_command::{CommandResult, InteractionCommand, OptionKind, Response, Target};
use crate::search::SearchCategory;

use super::{item_rows, object_embed, obtainable};

pub struct GetCommand;

impl InteractionCommand for GetCommand {
    const NAME: &'static str = "get";
    const DESCRIPTION: &'static str = "View how to get an item!";
    const OPTION_NAME: &'static str = "item";
    const OPTION_DESCRIPTION: &'static str = "An item in LEGO Universe (name or id).";
    const OPTION: OptionKind = OptionKind::Search(SearchCategory::Item);

    fn run(d: &GameData, target: Target) -> CommandResult {
        let item = CdClientObjectsId(target.id);
        item.try_fetch(d)?;
        let how = obtainable(d, item);
        let mark = |yes: bool| if yes { "✅" } else { "❌" };
        let description = [
            format!("{} Can be earned from a mission (`/earn`)", mark(how.earn)),
            format!("{} Can be dropped from a smashable (`/drop`)", mark(how.drop)),
            format!("{} Can be unpacked from a package (`/unpack`)", mark(how.unpack)),
            format!("{} Can be rewarded from an activity (`/reward`)", mark(how.reward)),
            format!("{} Can be bought from a vendor (`/buy`)", mark(how.buy)),
        ]
        .join("\n");
        let embed = object_embed(d, item).description(description);
        Ok(Response::new(embed).rows(item_rows(d, item, Self::NAME)))
    }
}
