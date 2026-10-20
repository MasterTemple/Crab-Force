use crate::data::GameData;
use crate::ids::{CdClientMissionsId, CdClientObjectsId, LUExplorer};
use crate::interaction_command::{select_row, CommandResult, InteractionCommand, OptionKind, Response, Target};
use crate::pager::Pager;
use crate::search::SearchCategory;

use super::mission::MissionCommand;
use super::{item_rows, numbered, object_embed};

pub struct EarnCommand;

impl InteractionCommand for EarnCommand {
    const NAME: &'static str = "earn";
    const DESCRIPTION: &'static str = "View all missions and achievements that reward an item!";
    const OPTION_NAME: &'static str = "item";
    const OPTION_DESCRIPTION: &'static str = "An item in LEGO Universe (name or id).";
    const OPTION: OptionKind = OptionKind::Search(SearchCategory::Item);

    fn run(d: &GameData, target: Target) -> CommandResult {
        let item = CdClientObjectsId(target.id);
        item.try_fetch(d)?;

        let mut missions = item.missions_rewarding(d);
        missions.sort_by_cached_key(|m| (m.category_path(d), m.req_name(d)));
        let pager = Pager::new(missions, target.page, 10);

        let mut embed = object_embed(d, item).title(format!("{}{}", item.name_id(d), pager.title_suffix()));
        let mut options = vec![];
        if pager.is_empty() {
            embed = embed.description(format!("{} is not rewarded by any mission.", item.hyperlink_name(d)));
        } else {
            let lines: Vec<String> = pager
                .this_page()
                .map(|(num, &mission)| {
                    let count = reward_count(d, mission, item.0);
                    let kind = if mission.is_mission(d) { "Mission" } else { "Achievement" };
                    options.push(MissionCommand::select_option(
                        mission.name_id(d),
                        Some(mission.category_path(d)),
                        Target::new(mission.0),
                    ));
                    numbered(
                        num,
                        format!(
                            "{} › {}{}",
                            mission.category_path(d),
                            mission.hyperlink_name(d),
                            if count > 1 { format!(" ×{count}") } else { String::new() }
                        ),
                    ) + &format!(" *({kind})*")
                })
                .collect();
            embed = embed.field_lines("Rewarded By", &lines, false);
        }

        Ok(Response::new(embed)
            .row(select_row(Self::NAME, "Open a mission…", options))
            .row(Self::page_row(&pager, target))
            .rows(item_rows(d, item, Self::NAME)))
    }
}

fn reward_count(d: &GameData, mission: CdClientMissionsId, item: i32) -> i32 {
    mission
        .rewards(d)
        .iter()
        .filter(|r| r.item == item)
        .map(|r| r.count)
        .max()
        .unwrap_or(1)
}
