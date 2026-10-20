use crate::data::GameData;
use crate::ids::{CdClientObjectsId, CdClientSkillBehaviorId, LUExplorer};
use crate::interaction_command::{select_row, CommandResult, InteractionCommand, OptionKind, Response, Target};
use crate::search::SearchCategory;

use super::skill::{skill_summary, SkillCommand};
use super::{item_rows, object_embed};

pub struct SkillsCommand;

impl InteractionCommand for SkillsCommand {
    const NAME: &'static str = "skills";
    const DESCRIPTION: &'static str = "View all skills attached to an item!";
    const OPTION_NAME: &'static str = "item";
    const OPTION_DESCRIPTION: &'static str = "An item in LEGO Universe (name or id).";
    const OPTION: OptionKind = OptionKind::Search(SearchCategory::Item);

    fn run(d: &GameData, target: Target) -> CommandResult {
        let item = CdClientObjectsId(target.id);
        item.try_fetch(d)?;

        let mut embed = object_embed(d, item);
        let mut options = vec![];
        let skills = item.skills(d);
        if skills.is_empty() {
            embed = embed.description(format!("{} has no skills.", item.hyperlink_name(d)));
        }
        for object_skill in skills {
            let skill = CdClientSkillBehaviorId(object_skill.skill);
            let mut value = skill_summary(d, skill);
            if object_skill.object != item.0 {
                value = format!("*From {}*\n{value}", CdClientObjectsId(object_skill.object).hyperlink_name(d));
            }
            // from the data: passive stat boosts use 1, attacks/abilities use 0
            let when = match object_skill.cast_on_type {
                Some(0) => " (on use)",
                Some(1) => " (on equip)",
                _ => "",
            };
            embed.push_field(format!("{}{when}", skill.name_id(d)), value, false);
            options.push(SkillCommand::select_option(skill.name_id(d), None, Target::new(skill.0)));
        }

        Ok(Response::new(embed)
            .row(select_row(Self::NAME, "Open a skill…", options))
            .rows(item_rows(d, item, Self::NAME)))
    }
}
