use crate::data::GameData;
use crate::error::BotError;
use crate::ids::{CdClientMissionsId, CdClientObjectsId, LUExplorer};
use crate::interaction_command::{select_row, CommandResult, InteractionCommand, OptionKind, Response, Target};
use crate::pager::Pager;
use crate::search::SearchCategory;

use super::mission::MissionCommand;
use super::{npc_row, object_embed};

pub struct NpcCommand;

impl InteractionCommand for NpcCommand {
    const NAME: &'static str = "npc";
    const DESCRIPTION: &'static str = "View all missions from an NPC!";
    const OPTION_NAME: &'static str = "npc";
    const OPTION_DESCRIPTION: &'static str = "An NPC in LEGO Universe (name or id).";
    const OPTION: OptionKind = OptionKind::Search(SearchCategory::Npc);

    fn run(d: &GameData, target: Target) -> CommandResult {
        let npc = CdClientObjectsId(target.id);
        npc.try_fetch(d)?;
        let missions = npc.npc_missions(d).to_vec();
        if missions.is_empty() && !npc.is_vendor(d) {
            return Err(BotError::not_applicable(format!(
                "{} does not offer or accept any missions.",
                npc.hyperlink_name(d)
            )));
        }
        let pager = Pager::new(missions, target.page, 8);

        let mut embed = object_embed(d, npc).title(format!("{}{}", npc.name_id(d), pager.title_suffix()));
        let mut options = vec![];
        if pager.is_empty() {
            embed = embed.description(format!("{} does not offer or accept any missions.", npc.hyperlink_name(d)));
        }
        for (num, entry) in pager.this_page() {
            let mission = CdClientMissionsId(entry.mission);
            let role = match (entry.offers, entry.accepts) {
                (true, true) => "Offers & accepts",
                (true, false) => "Offers",
                (false, true) => "Accepts",
                (false, false) => "Involved in",
            };
            let objective = mission.objective(d).unwrap_or_else(|| String::from("No description"));
            embed.push_field(
                format!("{num}. {}", mission.full_path(d)),
                format!("{objective}\n*{role}* {}", mission.hyperlink_name(d)),
                false,
            );
            options.push(MissionCommand::select_option(
                mission.name_id(d),
                Some(mission.category_path(d)),
                Target::new(mission.0),
            ));
        }

        Ok(Response::new(embed)
            .row(select_row(Self::NAME, "Open a mission…", options))
            .row(Self::page_row(&pager, target))
            .row(Some(npc_row(d, npc, Self::NAME))))
    }
}
