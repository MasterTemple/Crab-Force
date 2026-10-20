use crate::data::GameData;
use crate::ids::{CdClientObjectsId, LUExplorer};
use crate::interaction_command::{CommandResult, InteractionCommand, OptionKind, Response, Target};
use crate::search::SearchCategory;

use super::{item_rows, numbered, object_embed};

pub struct PreconditionsCommand;

impl InteractionCommand for PreconditionsCommand {
    const NAME: &'static str = "preconditions";
    const DESCRIPTION: &'static str = "View the preconditions to use an item!";
    const OPTION_NAME: &'static str = "item";
    const OPTION_DESCRIPTION: &'static str = "An item in LEGO Universe (name or id).";
    const OPTION: OptionKind = OptionKind::Search(SearchCategory::Item);

    fn run(d: &GameData, target: Target) -> CommandResult {
        let item = CdClientObjectsId(target.id);
        item.req_item_component(d)?;

        let lines: Vec<String> = item
            .preconditions(d)
            .into_iter()
            .enumerate()
            .map(|(idx, id)| {
                let reason = d
                    .locale
                    .preconditions
                    .get(&id)
                    .and_then(|p| p.failure_reason.clone())
                    .unwrap_or_else(|| format!("Precondition {id}"));
                numbered(idx + 1, format!("{reason} `[{id}]`"))
            })
            .collect();

        let mut embed = object_embed(d, item);
        if lines.is_empty() {
            embed = embed.description(format!("{} can be used by anyone.", item.hyperlink_name(d)));
        } else {
            embed = embed.field_lines("Preconditions", &lines, false);
        }
        Ok(Response::new(embed).rows(item_rows(d, item, Self::NAME)))
    }
}
