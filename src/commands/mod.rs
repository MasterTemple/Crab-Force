//! Every slash command, plus the shared pieces they use

pub mod achievement;
pub mod activity;
pub mod brick;
pub mod buy;
pub mod cooldowngroup;
pub mod drop;
pub mod earn;
pub mod enemy;
pub mod get;
pub mod item;
pub mod level;
pub mod loottable;
pub mod mission;
pub mod npc;
pub mod package;
pub mod preconditions;
pub mod reload;
pub mod reward;
pub mod search;
pub mod skill;
pub mod skillitems;
pub mod skills;
pub mod smash;
pub mod unpack;
pub mod vendor;

use serenity::all::{CreateActionRow, CreateCommand, ResolvedOption};

use crate::cdclient::components::DESTROYABLE_COMPONENT;
use crate::cdclient::CdClientDestructibleComponent;
use crate::data::GameData;
use crate::embed::Embed;
use crate::error::BotResult;
use crate::ids::{CdClientObjectsId, CdClientSkillBehaviorId, LUExplorer};
use crate::interaction_command::{
    select_row, CommandResult, InteractionCommand, OptionKind, SelectOption, Target,
};
use crate::loot::{format_chance, format_number, format_range, loot_rolls};

use self::achievement::AchievementCommand;
use self::activity::ActivityCommand;
use self::brick::BrickCommand;
use self::buy::BuyCommand;
use self::cooldowngroup::CooldownGroupCommand;
use self::drop::DropCommand;
use self::earn::EarnCommand;
use self::enemy::EnemyCommand;
use self::get::GetCommand;
use self::item::ItemCommand;
use self::level::LevelCommand;
use self::loottable::LootTableCommand;
use self::mission::MissionCommand;
use self::npc::NpcCommand;
use self::package::PackageCommand;
use self::preconditions::PreconditionsCommand;
use self::reward::RewardCommand;
use self::skill::SkillCommand;
use self::skillitems::SkillItemsCommand;
use self::skills::SkillsCommand;
use self::smash::SmashCommand;
use self::unpack::UnpackCommand;
use self::vendor::VendorCommand;

// -------- //
// Registry //
// -------- //

/// A command that follows the standard [`InteractionCommand`] shape
pub struct CommandSpec {
    pub name: &'static str,
    pub option: OptionKind,
    pub register: fn() -> CreateCommand,
    pub run: fn(&GameData, Target) -> CommandResult,
    pub target_from_options: fn(&GameData, &[ResolvedOption]) -> BotResult<Target>,
}

macro_rules! spec {
    ($cmd:ty) => {
        CommandSpec {
            name: <$cmd>::NAME,
            option: <$cmd>::OPTION,
            register: <$cmd>::register,
            run: <$cmd>::run,
            target_from_options: <$cmd>::target_from_options,
        }
    };
}

pub static COMMANDS: &[CommandSpec] = &[
    spec!(AchievementCommand),
    spec!(ActivityCommand),
    spec!(BrickCommand),
    spec!(BuyCommand),
    spec!(CooldownGroupCommand),
    spec!(DropCommand),
    spec!(EarnCommand),
    spec!(EnemyCommand),
    spec!(GetCommand),
    spec!(ItemCommand),
    spec!(LevelCommand),
    spec!(LootTableCommand),
    spec!(MissionCommand),
    spec!(NpcCommand),
    spec!(PackageCommand),
    spec!(PreconditionsCommand),
    spec!(RewardCommand),
    spec!(SkillCommand),
    spec!(SkillItemsCommand),
    spec!(SkillsCommand),
    spec!(SmashCommand),
    spec!(UnpackCommand),
    spec!(VendorCommand),
];

pub fn find(name: &str) -> Option<&'static CommandSpec> {
    COMMANDS.iter().find(|spec| spec.name == name)
}

/// Every command to register with Discord
pub fn registrations() -> Vec<CreateCommand> {
    COMMANDS
        .iter()
        .map(|spec| (spec.register)())
        .chain([search::register(), reload::register()])
        .collect()
}

// ------- //
// Headers //
// ------- //

/// An embed titled `Name [id]` linking to LU Explorer, with the object's icon
pub fn object_embed(d: &GameData, object: CdClientObjectsId) -> Embed {
    Embed::new()
        .title(object.name_id(d))
        .url(object.explorer_url())
        .thumbnail(object.thumbnail(d))
}

// ---------------- //
// Navigation Rows //
// ---------------- //

/// Which ways an item can be obtained
pub struct Obtainable {
    pub earn: bool,
    pub drop: bool,
    pub unpack: bool,
    pub reward: bool,
    pub buy: bool,
}

pub fn obtainable(d: &GameData, item: CdClientObjectsId) -> Obtainable {
    let lmis = item.loot_matrices(d);
    let used_by = |map: &std::collections::HashMap<i32, Vec<i32>>| lmis.iter().any(|lmi| map.contains_key(lmi));
    Obtainable {
        earn: !item.missions_rewarding(d).is_empty(),
        drop: used_by(&d.index.destructibles_with_matrix),
        unpack: used_by(&d.index.packages_with_matrix),
        reward: used_by(&d.index.activities_with_matrix),
        buy: !item.vendors(d).is_empty()
            || item.item_component(d).is_some_and(|ic| ic.commendation_cost.unwrap_or(0) > 0),
    }
}

/// The two rows of item buttons: how to get it, and information about it
pub fn item_rows(d: &GameData, item: CdClientObjectsId, current: &str) -> Vec<CreateActionRow> {
    let id = item.0;
    let how = obtainable(d, item);
    let is_item = item.item_component(d).is_some();
    vec![
        CreateActionRow::Buttons(vec![
            EarnCommand::nav_button("Earn", id, current, how.earn),
            DropCommand::nav_button("Drop", id, current, how.drop),
            UnpackCommand::nav_button("Unpack", id, current, how.unpack),
            RewardCommand::nav_button("Reward", id, current, how.reward),
            BuyCommand::nav_button("Buy", id, current, how.buy),
        ]),
        CreateActionRow::Buttons(vec![
            ItemCommand::nav_button("Item", id, current, is_item),
            GetCommand::nav_button("Get", id, current, is_item),
            PreconditionsCommand::nav_button("Preconditions", id, current, !item.preconditions(d).is_empty()),
            SkillsCommand::nav_button("Skills", id, current, !item.skills(d).is_empty()),
        ]),
    ]
}

/// Buttons for something that can be smashed
pub fn enemy_row(d: &GameData, object: CdClientObjectsId, current: &str) -> CreateActionRow {
    let id = object.0;
    CreateActionRow::Buttons(vec![
        EnemyCommand::nav_button("Stats", id, current, object.destructible_component(d).is_some()),
        SmashCommand::nav_button("Drops", id, current, object.smash_loot_matrix(d).is_some()),
    ])
}

/// Buttons for an NPC
pub fn npc_row(d: &GameData, object: CdClientObjectsId, current: &str) -> CreateActionRow {
    let id = object.0;
    CreateActionRow::Buttons(vec![
        NpcCommand::nav_button("Missions", id, current, !object.npc_missions(d).is_empty()),
        VendorCommand::nav_button("Sells", id, current, object.vendor_loot_matrix(d).is_some()),
    ])
}

/// Buttons for a skill
pub fn skill_row(d: &GameData, skill: CdClientSkillBehaviorId, current: &str) -> CreateActionRow {
    let id = skill.0;
    let mut buttons = vec![
        SkillCommand::nav_button("Skill", id, current, skill.fetch(d).is_some()),
        SkillItemsCommand::nav_button("Items", id, current, !skill.objects(d).is_empty()),
    ];
    if let Some(group) = skill.fetch(d).and_then(|s| s.cooldowngroup) {
        buttons.push(CooldownGroupCommand::nav_button(format!("Cooldown Group {group}"), group, current, true));
    }
    CreateActionRow::Buttons(buttons)
}

// ------------ //
// Loot Display //
// ------------ //

/// Adds one field per roll of a loot matrix, and returns a menu to open its loot tables
pub fn add_loot_matrix(d: &GameData, embed: Embed, loot_matrix: i32, menu_id: &str) -> (Embed, Option<CreateActionRow>) {
    let mut embed = embed;
    let rolls = loot_rolls(d, loot_matrix);
    if rolls.is_empty() {
        return (embed.field("Drops", "Nothing", false), None);
    }
    let mut options = vec![];
    for (num, roll) in rolls.iter().enumerate() {
        let items = if roll.max == 1 { "item" } else { "items" };
        let name = format!(
            "{}. {} chance for {} {items}",
            num + 1,
            crate::config::config().percent(roll.percent),
            format_range(roll.min, roll.max)
        );
        let mut lines = vec![format!("From {}", roll.loot_table.hyperlink_name(d))];
        for &(rarity, chance, count) in &roll.tiers {
            lines.push(format!(
                "Tier {rarity}: `{}` for one of {count} item{} (`{}` each)",
                crate::config::config().percent(chance),
                if count == 1 { "" } else { "s" },
                crate::config::config().percent(chance / count as f64),
            ));
        }
        if roll.tiers.is_empty() && roll.unrated > 0 {
            lines.push(format!(
                "One of {} item{} (`{}` each)",
                roll.unrated,
                if roll.unrated == 1 { "" } else { "s" },
                crate::config::config().percent(1.0 / roll.unrated as f64),
            ));
        }
        embed.push_field(name, lines.join("\n"), false);
        options.push(LootTableCommand::select_option(
            roll.loot_table.name_id(d),
            Some(format!("{} items", roll.loot_table.items(d).len())),
            Target::new(roll.loot_table.0),
        ));
    }
    (embed, select_row(menu_id, "View a loot table…", options))
}

/// Coins dropped, using the currency table row for the object's level
pub fn coin_range(d: &GameData, currency_index: Option<i32>, level: Option<i32>) -> Option<String> {
    let index = currency_index?;
    let level = level.unwrap_or(1);
    let row = d
        .cdclient
        .currency_table
        .iter()
        .filter(|row| row.currency_index == index && row.npcminlevel <= level)
        .max_by_key(|row| row.npcminlevel)?;
    if row.maxvalue <= 0 {
        return None;
    }
    Some(format!(
        "{} coins",
        if row.minvalue == row.maxvalue {
            format_number(row.minvalue.into())
        } else {
            format!("{}–{}", format_number(row.minvalue.into()), format_number(row.maxvalue.into()))
        }
    ))
}

pub fn smash_coins(d: &GameData, dc: &CdClientDestructibleComponent) -> Option<String> {
    coin_range(d, dc.currency_index, dc.level)
}

/// The command that best describes an object (used by `/search` and menus)
pub fn object_command(d: &GameData, object: CdClientObjectsId) -> Option<(&'static str, &'static str)> {
    if object.item_component(d).is_some() {
        Some((ItemCommand::NAME, "item"))
    } else if object.is_package(d) {
        Some((PackageCommand::NAME, "package"))
    } else if object.is_vendor(d) {
        Some((VendorCommand::NAME, "vendor"))
    } else if object.is_npc(d) {
        Some((NpcCommand::NAME, "NPC"))
    } else if object.has_component(d, DESTROYABLE_COMPONENT) {
        Some((EnemyCommand::NAME, "smashable"))
    } else {
        None
    }
}

/// A select option for an object, opening whatever command describes it best
pub fn object_select_option(d: &GameData, object: CdClientObjectsId, description: Option<String>) -> Option<SelectOption> {
    let (command, _) = object_command(d, object)?;
    Some(SelectOption {
        label: object.name_id(d),
        description,
        value: crate::interaction_command::CustomId::encode(
            crate::interaction_command::Mode::Reply,
            command,
            Target::new(object.0),
            None,
        ),
    })
}

/// A line like `**1.** Name [`[id]`](url)`
pub fn numbered(num: usize, text: impl std::fmt::Display) -> String {
    format!("**{num}.** {text}")
}

/// Chance shown for drops/unpacks/rewards
pub fn chance_text(chance: f64) -> String {
    format_chance(chance)
}
