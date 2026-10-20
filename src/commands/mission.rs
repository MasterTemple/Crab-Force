use crate::data::GameData;
use crate::interaction_command::{CommandResult, InteractionCommand, OptionKind, Target};
use crate::search::SearchCategory;

use super::achievement::mission_response;

pub struct MissionCommand;

impl InteractionCommand for MissionCommand {
    const NAME: &'static str = "mission";
    const DESCRIPTION: &'static str = "View the stats of a mission!";
    const OPTION_NAME: &'static str = "mission";
    const OPTION_DESCRIPTION: &'static str = "A mission in LEGO Universe (name or id).";
    const OPTION: OptionKind = OptionKind::Search(SearchCategory::Mission);

    fn run(d: &GameData, target: Target) -> CommandResult {
        mission_response(d, target)
    }
}
