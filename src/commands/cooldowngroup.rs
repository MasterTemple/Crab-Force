use crate::data::GameData;
use crate::embed::Embed;
use crate::ids::{CdClientSkillBehaviorId, CooldownGroupId, LUExplorer};
use crate::interaction_command::{select_row, CommandResult, InteractionCommand, OptionKind, Response, Target};
use crate::pager::Pager;
use crate::search::SearchCategory;

use super::skill::{skill_summary, SkillCommand};

pub struct CooldownGroupCommand;

impl InteractionCommand for CooldownGroupCommand {
    const NAME: &'static str = "cooldowngroup";
    const DESCRIPTION: &'static str = "View the skills in a cooldown group!";
    const OPTION_NAME: &'static str = "group";
    const OPTION_DESCRIPTION: &'static str = "A cooldown group number (or the name of a skill in it).";
    const OPTION: OptionKind = OptionKind::Search(SearchCategory::CooldownGroup);

    fn run(d: &GameData, target: Target) -> CommandResult {
        let group = CooldownGroupId(target.id);
        let skills = group.skills(d);
        if skills.is_empty() {
            return Err(group.not_found(d));
        }
        let pager = Pager::new(skills.to_vec(), target.page, 10);

        let mut embed = Embed::new()
            .title(format!("Cooldown Group {}{}", group.0, pager.title_suffix()))
            .url(group.explorer_url())
            .description(format!("Using any of these {} skills puts the others on cooldown.", pager.total()));
        let mut options = vec![];
        for (_, &skill) in pager.this_page() {
            let skill = CdClientSkillBehaviorId(skill);
            embed.push_field(skill.name_id(d), skill_summary(d, skill), true);
            options.push(SkillCommand::select_option(skill.name_id(d), None, Target::new(skill.0)));
        }

        Ok(Response::new(embed)
            .row(select_row(Self::NAME, "Open a skill…", options))
            .row(Self::page_row(&pager, target)))
    }
}
