use serenity::all::{CommandOptionType, CreateCommand, CreateCommandOption, ResolvedOption, ResolvedValue};

use crate::data::GameData;
use crate::embed::Embed;
use crate::error::{BotError, BotResult};
use crate::ids::{
    explorer_hyperlink, CdClientActivityId, CdClientLootTableId, CdClientMissionsId, CdClientObjectsId,
    CdClientSkillBehaviorId, CooldownGroupId, LUExplorer,
};
use crate::interaction_command::{select_row, CommandResult, CustomId, Mode, Response, SelectOption, Target};
use crate::search::{sanitize_inline, SearchCategory, SearchHit};


pub const NAME: &str = "search";

pub fn register() -> CreateCommand {
    let mut category = CreateCommandOption::new(CommandOptionType::String, "category", "Only search one kind of thing.")
        .required(false);
    for c in SearchCategory::ALL {
        let mut label = c.label().to_string();
        if let Some(first) = label.get_mut(0..1) {
            first.make_ascii_uppercase();
        }
        category = category.add_string_choice(label, c.key());
    }
    CreateCommand::new(NAME)
        .description("Search everything in LEGO Universe by name or id!")
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "query", "What to search for (typos are okay).")
                .required(true)
                .max_length(100),
        )
        .add_option(category)
}

pub fn parse_options(options: &[ResolvedOption]) -> BotResult<(String, Option<SearchCategory>)> {
    let mut query = None;
    let mut category = None;
    for option in options {
        match (option.name, &option.value) {
            ("query", ResolvedValue::String(q)) => query = Some(q.to_string()),
            ("category", ResolvedValue::String(c)) => {
                category = Some(
                    SearchCategory::from_key(c)
                        .ok_or_else(|| BotError::invalid(format!("Unknown category `{}`.", sanitize_inline(c))))?,
                )
            }
            _ => {}
        }
    }
    let query = query.filter(|q| !q.trim().is_empty()).ok_or_else(|| BotError::invalid("Please enter something to search for."))?;
    Ok((query, category))
}

/// The command that opens a search result, and its hyperlinked name
fn describe(d: &GameData, category: SearchCategory, id: i32) -> (Option<&'static str>, String) {
    use super::{
        achievement, activity, brick, cooldowngroup, enemy, item, loottable, mission, npc, object_command, package,
        skill, smash, vendor, InteractionCommand,
    };
    match category {
        SearchCategory::Object => {
            let object = CdClientObjectsId(id);
            (object_command(d, object).map(|c| c.0), object.hyperlink_name(d))
        }
        SearchCategory::Item => (Some(item::ItemCommand::NAME), CdClientObjectsId(id).hyperlink_name(d)),
        SearchCategory::Brick => (Some(brick::BrickCommand::NAME), CdClientObjectsId(id).hyperlink_name(d)),
        SearchCategory::Enemy => (Some(enemy::EnemyCommand::NAME), CdClientObjectsId(id).hyperlink_name(d)),
        SearchCategory::Smashable => (Some(smash::SmashCommand::NAME), CdClientObjectsId(id).hyperlink_name(d)),
        SearchCategory::Package => (Some(package::PackageCommand::NAME), CdClientObjectsId(id).hyperlink_name(d)),
        SearchCategory::Vendor => (Some(vendor::VendorCommand::NAME), CdClientObjectsId(id).hyperlink_name(d)),
        SearchCategory::Npc => (Some(npc::NpcCommand::NAME), CdClientObjectsId(id).hyperlink_name(d)),
        SearchCategory::Mission => (Some(mission::MissionCommand::NAME), CdClientMissionsId(id).hyperlink_name(d)),
        SearchCategory::Achievement => (Some(achievement::AchievementCommand::NAME), CdClientMissionsId(id).hyperlink_name(d)),
        SearchCategory::Skill => (Some(skill::SkillCommand::NAME), CdClientSkillBehaviorId(id).hyperlink_name(d)),
        SearchCategory::Activity => (Some(activity::ActivityCommand::NAME), CdClientActivityId(id).hyperlink_name(d)),
        SearchCategory::LootTable => (Some(loottable::LootTableCommand::NAME), CdClientLootTableId(id).hyperlink_name(d)),
        SearchCategory::CooldownGroup => {
            let group = CooldownGroupId(id);
            (
                Some(cooldowngroup::CooldownGroupCommand::NAME),
                explorer_hyperlink(format!("Cooldown Group {id}"), id, group.explorer_url()),
            )
        }
    }
}

pub fn run(d: &GameData, query: &str, category: Option<SearchCategory>) -> CommandResult {
    // the specific categories cover every object worth opening, so `Object` is only a fallback
    let categories: Vec<SearchCategory> = match category {
        Some(c) => vec![c],
        None => SearchCategory::ALL.into_iter().filter(|&c| c != SearchCategory::Object).collect(),
    };
    let per_category = if category.is_some() { 25 } else { 5 };

    let mut results: Vec<(SearchCategory, Vec<SearchHit>)> = categories
        .into_iter()
        .map(|c| (c, d.search.search(c, query, per_category)))
        .filter(|(_, hits)| !hits.is_empty())
        .collect();
    if results.is_empty() && category.is_none() {
        let hits = d.search.search(SearchCategory::Object, query, 10);
        if !hits.is_empty() {
            results.push((SearchCategory::Object, hits));
        }
    }
    // best categories first
    results.sort_by_key(|(_, hits)| std::cmp::Reverse(hits.first().map_or(0, |h| h.score)));

    let mut embed = Embed::new().title(format!("Search: {}", query.trim()));
    if results.is_empty() {
        embed = embed.description(format!(
            "Nothing matches `{}`. Try fewer or different words, or an id.",
            sanitize_inline(query)
        ));
        return Ok(Response::new(embed));
    }

    let mut options = vec![];
    for (category, hits) in &results {
        let mut lines = vec![];
        for hit in hits {
            let (command, link) = describe(d, *category, hit.entry.id);
            let detail = hit
                .entry
                .detail
                .as_ref()
                .map(|detail| format!(" · *{}*", crate::ids::escape_markdown(detail)))
                .unwrap_or_default();
            lines.push(format!("- {link}{detail}"));
            if let Some(command) = command {
                options.push(SelectOption {
                    label: hit.entry.label(),
                    description: Some(format!("Open with /{command}")),
                    value: CustomId::encode(Mode::Reply, command, Target::new(hit.entry.id), None),
                });
            }
        }
        let mut name = category.label().to_string();
        if let Some(first) = name.get_mut(0..1) {
            first.make_ascii_uppercase();
        }
        let total = d.search.len(*category);
        embed = embed.field_lines(format!("{name} ({} of {total})", hits.len()), &lines, false);
    }

    // put the overall best matches first in the menu
    Ok(Response::new(embed).row(select_row(NAME, "Open a result…", options)))
}
