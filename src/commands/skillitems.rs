use crate::data::GameData;
use crate::embed::Embed;
use crate::ids::{CdClientObjectsId, CdClientSkillBehaviorId, LUExplorer};
use crate::interaction_command::{select_row, CommandResult, InteractionCommand, OptionKind, Response, Target};
use crate::pager::Pager;
use crate::search::SearchCategory;

use super::{numbered, object_select_option, skill_row};

pub struct SkillItemsCommand;

impl InteractionCommand for SkillItemsCommand {
    const NAME: &'static str = "skillitems";
    const DESCRIPTION: &'static str = "View all items that have a skill!";
    const OPTION_NAME: &'static str = "skill";
    const OPTION_DESCRIPTION: &'static str = "A skill in LEGO Universe (name or id).";
    const OPTION: OptionKind = OptionKind::Search(SearchCategory::Skill);

    fn run(d: &GameData, target: Target) -> CommandResult {
        let id = CdClientSkillBehaviorId(target.id);
        id.try_fetch(d)?;

        let objects: Vec<CdClientObjectsId> = id
            .objects(d)
            .iter()
            .copied()
            .filter(|&o| d.is_visible(o))
            .map(CdClientObjectsId)
            .collect();
        let pager = Pager::new(objects, target.page, 20);

        let mut embed = Embed::new()
            .title(format!("{}{}", id.name_id(d), pager.title_suffix()))
            .url(id.explorer_url())
            .thumbnail(id.thumbnail(d));
        let mut options = vec![];
        if pager.is_empty() {
            embed = embed.description(format!("No objects have {}.", id.hyperlink_name(d)));
        } else {
            let lines: Vec<String> = pager
                .this_page()
                .map(|(num, &object)| {
                    options.extend(object_select_option(d, object, None));
                    numbered(num, object.hyperlink_name(d))
                })
                .collect();
            embed = embed.field_lines(format!("{} Objects", pager.total()), &lines, false);
        }

        Ok(Response::new(embed)
            .row(select_row(Self::NAME, "Open an object…", options))
            .row(Self::page_row(&pager, target))
            .row(Some(skill_row(d, id, Self::NAME))))
    }
}
