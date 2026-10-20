use crate::data::GameData;
use crate::error::BotError;
use crate::ids::{CdClientObjectsId, LUExplorer};
use crate::interaction_command::{CommandResult, InteractionCommand, OptionKind, Response, Target};
use crate::search::SearchCategory;

use super::{add_loot_matrix, item_rows, object_embed};

pub struct PackageCommand;

impl InteractionCommand for PackageCommand {
    const NAME: &'static str = "package";
    const DESCRIPTION: &'static str = "View all items given from a package!";
    const OPTION_NAME: &'static str = "package";
    const OPTION_DESCRIPTION: &'static str = "A package in LEGO Universe (name or id).";
    const OPTION: OptionKind = OptionKind::Search(SearchCategory::Package);

    fn run(d: &GameData, target: Target) -> CommandResult {
        let package = CdClientObjectsId(target.id);
        package.try_fetch(d)?;
        let lmi = package.package_loot_matrix(d).ok_or_else(|| {
            BotError::not_applicable(format!("{} is not a package.", package.hyperlink_name(d)))
        })?;
        let (embed, menu) = add_loot_matrix(d, object_embed(d, package), lmi, Self::NAME);
        let mut response = Response::new(embed).row(menu);
        if package.item_component(d).is_some() {
            response = response.rows(item_rows(d, package, Self::NAME));
        }
        Ok(response)
    }
}
