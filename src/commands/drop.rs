use crate::data::GameData;
use crate::ids::{escape_markdown, CdClientActivityId, CdClientObjectsId, LUExplorer};
use crate::interaction_command::{select_row, CommandResult, InteractionCommand, OptionKind, Response, Target};
use crate::loot::{item_sources, LootSource};
use crate::pager::Pager;
use crate::search::SearchCategory;

use super::activity::{activity_page, ActivityCommand};
use super::{chance_text, item_rows, object_embed, object_select_option};

pub struct DropCommand;

impl InteractionCommand for DropCommand {
    const NAME: &'static str = "drop";
    const DESCRIPTION: &'static str = "View all smashables that drop an item!";
    const OPTION_NAME: &'static str = "item";
    const OPTION_DESCRIPTION: &'static str = "An item in LEGO Universe (name or id).";
    const OPTION: OptionKind = OptionKind::Search(SearchCategory::Item);

    fn run(d: &GameData, target: Target) -> CommandResult {
        item_sources_response::<Self>(d, target, LootSource::Smashable)
    }
}

/// Shared by `/drop`, `/unpack`, and `/reward`: where an item comes from and how likely it is
pub fn item_sources_response<C: InteractionCommand>(d: &GameData, target: Target, kind: LootSource) -> CommandResult {
    const PAGE_SIZE: usize = 6;
    let item = CdClientObjectsId(target.id);
    item.try_fetch(d)?;
    let name = item.req_name(d);

    let groups = item_sources(d, item, kind);
    let (verb, source_kind) = match kind {
        LootSource::Smashable => ("dropped by", "smashable"),
        LootSource::Package => ("unpacked from", "package"),
        LootSource::Activity => ("rewarded by", "activity"),
    };
    let pager = Pager::new(groups, target.page, PAGE_SIZE);

    let mut embed = object_embed(d, item).title(format!("{}{}", item.name_id(d), pager.title_suffix()));
    let mut options = vec![];
    if pager.is_empty() {
        embed = embed.description(format!("{} is not {verb} any {source_kind}.", item.hyperlink_name(d)));
    } else {
        embed = embed.description(format!(
            "Chance to get at least one **{}** each time.",
            escape_markdown(&name)
        ));
        for (num, group) in pager.this_page() {
            let mut lines = vec![];
            for source in &group.sources {
                let (link, option) = match kind {
                    LootSource::Activity => {
                        let activity = CdClientActivityId(source.id);
                        let (page, reward) = activity_page(d, activity, source.loot_matrix);
                        let detail = reward.map(|r| format!(" — *{}*", escape_markdown(&r))).unwrap_or_default();
                        (
                            format!("{}{detail}", activity.hyperlink_name(d)),
                            Some(ActivityCommand::select_option(activity.name_id(d), None, Target::new(source.id).page(page))),
                        )
                    }
                    _ => {
                        let object = CdClientObjectsId(source.id);
                        (object.hyperlink_name(d), object_select_option(d, object, None))
                    }
                };
                lines.push(format!("- {link}"));
                if let Some(mut option) = option {
                    option.description = Some(format!("{} chance", crate::config::config().percent(group.chance)));
                    options.push(option);
                }
            }
            let field = format!("{}. {}", num, chance_text(group.chance));
            embed = embed.field_lines(field, &lines, false);
        }
    }

    Ok(Response::new(embed)
        .row(select_row(C::NAME, &format!("Open a {source_kind}…"), options))
        .row(C::page_row(&pager, target))
        .rows(item_rows(d, item, C::NAME)))
}

