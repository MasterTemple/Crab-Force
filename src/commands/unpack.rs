use crate::data::GameData;
use crate::interaction_command::{CommandResult, InteractionCommand, OptionKind, Target};
use crate::loot::LootSource;
use crate::search::SearchCategory;

use super::drop::item_sources_response;

pub struct UnpackCommand;

impl InteractionCommand for UnpackCommand {
    const NAME: &'static str = "unpack";
    const DESCRIPTION: &'static str = "View all packages that contain an item!";
    const OPTION_NAME: &'static str = "item";
    const OPTION_DESCRIPTION: &'static str = "An item in LEGO Universe (name or id).";
    const OPTION: OptionKind = OptionKind::Search(SearchCategory::Item);

    fn run(d: &GameData, target: Target) -> CommandResult {
        item_sources_response::<Self>(d, target, LootSource::Package)
    }
}
