//! Typed ids for cdclient rows and the queries on them
//!
//! - Each id is a newtype so the wrong kind of id cannot be passed by accident
//! - Every query takes the `&GameData` snapshot it should read from

use std::fmt::Display;

use crate::cdclient::components::{
    DESTROYABLE_COMPONENT, ITEM_COMPONENT, MISSION_OFFER_COMPONENT, PACKAGE_COMPONENT,
    RENDER_COMPONENT, VENDOR_COMPONENT,
};
use crate::cdclient::{
    CdClientActivityRewards, CdClientDestructibleComponent, CdClientItemComponent, CdClientMissions,
    CdClientObjects, CdClientRenderComponent, CdClientSkillBehavior,
};
use crate::config::config;
use crate::data::GameData;
use crate::error::{BotError, BotResult};
use crate::locale::SkillBehaviorDescriptionUI;

/// Escapes characters that Discord markdown would otherwise interpret
pub fn escape_markdown(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for ch in text.chars() {
        if matches!(ch, '*' | '_' | '~' | '`' | '|' | '[' | ']' | '\\' | '>') {
            out.push('\\');
        }
        out.push(ch);
    }
    out
}

pub fn fmt_name_id(name: impl Display, id: impl Display) -> String {
    format!("{name} [{id}]")
}

/// `Name [`[id]`](url)`: the id is the link so names stay readable
pub fn explorer_hyperlink(name: impl Display, id: impl Display, explorer_url: impl Display) -> String {
    format!("{} [`[{id}]`]({explorer_url})", escape_markdown(&name.to_string()))
}

pub fn icon_asset_as_url(asset: impl AsRef<str>) -> String {
    config().explorer_res_uri(asset.as_ref())
}

pub fn icon_url(d: &GameData, icon_id: i32) -> Option<String> {
    let icon = d.cdclient.icons.at_key(&icon_id)?;
    Some(icon_asset_as_url(icon.icon_path.as_ref()?))
}

/// Something that has a page on LU Explorer
pub trait LUExplorer {
    /// e.g. "Object"
    const NAME: &'static str;
    /// **DO NOT PUT FRONT/BACK SLASHES**
    const ENDPOINT: &'static str;

    fn id(&self) -> i32;

    fn name(&self, _d: &GameData) -> Option<String> {
        None
    }

    fn req_name(&self, d: &GameData) -> String {
        self.name(d)
            .unwrap_or_else(|| format!("{} {}", Self::NAME, self.id()))
    }

    fn name_id(&self, d: &GameData) -> String {
        fmt_name_id(self.req_name(d), self.id())
    }

    fn explorer_url(&self) -> String {
        config().explorer_uri(format!("{}/{}", Self::ENDPOINT, self.id()))
    }

    fn hyperlink_name(&self, d: &GameData) -> String {
        explorer_hyperlink(self.req_name(d), self.id(), self.explorer_url())
    }

    /// `"<Name [id]> {msg}"` as a [`BotError::NotFound`]
    fn not_found(&self, d: &GameData) -> BotError {
        BotError::not_found(format!(
            "{} `{}` does not exist{}.",
            Self::NAME,
            self.id(),
            self.name(d).map(|n| format!(" ({n})")).unwrap_or_default()
        ))
    }
}

// ------- //
// Objects //
// ------- //

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CdClientObjectsId(pub i32);

impl LUExplorer for CdClientObjectsId {
    const NAME: &'static str = "Object";
    const ENDPOINT: &'static str = "objects";

    fn id(&self) -> i32 {
        self.0
    }

    /// The in-game (locale) name, then the cdclient display name, then the internal name
    fn name(&self, d: &GameData) -> Option<String> {
        if let Some(name) = d.locale.objects.get(&self.0).and_then(|o| o.name.clone()) {
            return Some(name);
        }
        let object = d.cdclient.objects.at_key(&self.0)?;
        object.display_name.clone().or_else(|| object.name.clone())
    }
}

/// An item's equip location in plain words
pub fn equip_location_name(raw: &str) -> String {
    match raw.trim() {
        "clavicle" => String::from("Armor"),
        "hair" => String::from("Head"),
        "legs" => String::from("Legs"),
        "chest" => String::from("Chest"),
        "special_r" => String::from("Right Hand"),
        "special_l" => String::from("Left Hand"),
        other => heck::ToTitleCase::to_title_case(other),
    }
}

/// A skill attached to an object (an item's own skills or one of its proxies')
#[derive(Clone, Copy, Debug)]
pub struct ObjectSkill {
    pub object: i32,
    pub skill: i32,
    pub cast_on_type: Option<i32>,
}

impl CdClientObjectsId {
    pub fn fetch<'a>(&self, d: &'a GameData) -> Option<&'a CdClientObjects> {
        d.cdclient.objects.at_key(&self.0)
    }

    /// The object, or a friendly "does not exist" error
    pub fn try_fetch<'a>(&self, d: &'a GameData) -> BotResult<&'a CdClientObjects> {
        let object = self.fetch(d).ok_or_else(|| self.not_found(d))?;
        if !d.is_visible(self.0) {
            return Err(self.not_found(d));
        }
        Ok(object)
    }

    pub fn description(&self, d: &GameData) -> Option<String> {
        d.locale
            .objects
            .get(&self.0)
            .and_then(|o| o.description.clone())
            .or_else(|| self.fetch(d)?.description.clone())
    }

    /// All component ids of one type registered to this object
    pub fn component_ids(&self, d: &GameData, component_type: i32) -> Vec<i32> {
        d.cdclient
            .components_registry
            .at_group_key(&self.0)
            .unwrap_or_default()
            .iter()
            .filter(|cr| cr.component_type == component_type)
            .map(|cr| cr.component_id)
            .collect()
    }

    pub fn component_id(&self, d: &GameData, component_type: i32) -> Option<i32> {
        self.component_ids(d, component_type).first().copied()
    }

    pub fn has_component(&self, d: &GameData, component_type: i32) -> bool {
        self.component_id(d, component_type).is_some()
    }

    pub fn item_component<'a>(&self, d: &'a GameData) -> Option<&'a CdClientItemComponent> {
        d.cdclient.item_component.at_key(&self.component_id(d, ITEM_COMPONENT)?)
    }

    /// The item component, or an error explaining the object is not an item
    pub fn req_item_component<'a>(&self, d: &'a GameData) -> BotResult<&'a CdClientItemComponent> {
        self.try_fetch(d)?;
        self.item_component(d).ok_or_else(|| {
            BotError::not_applicable(format!("{} is not an item.", self.hyperlink_name(d)))
        })
    }

    pub fn render_component<'a>(&self, d: &'a GameData) -> Option<&'a CdClientRenderComponent> {
        d.cdclient.render_component.at_key(&self.component_id(d, RENDER_COMPONENT)?)
    }

    pub fn destructible_component<'a>(&self, d: &'a GameData) -> Option<&'a CdClientDestructibleComponent> {
        d.cdclient
            .destructible_component
            .at_key(&self.component_id(d, DESTROYABLE_COMPONENT)?)
    }

    pub fn thumbnail(&self, d: &GameData) -> Option<String> {
        let render = self.render_component(d)?;
        render
            .icon_asset
            .as_ref()
            .map(icon_asset_as_url)
            .or_else(|| icon_url(d, render.icon_id?))
    }

    pub fn rarity(&self, d: &GameData) -> Option<i32> {
        d.index.item_rarity.get(&self.0).copied()
    }

    /// Items listed in the item component's `subItems` (proxies)
    pub fn proxies(&self, d: &GameData) -> Vec<CdClientObjectsId> {
        self.item_component(d)
            .and_then(|ic| ic.sub_items.clone())
            .unwrap_or_default()
            .into_iter()
            .filter(|&id| id > 0)
            .map(CdClientObjectsId)
            .collect()
    }

    /// Equip locations of the item and its proxies, e.g. `["Right Hand", "Left Hand"]`
    pub fn equip_locations(&self, d: &GameData) -> Vec<String> {
        let mut locations = vec![];
        for object in std::iter::once(*self).chain(self.proxies(d)) {
            if let Some(location) = object.item_component(d).and_then(|ic| ic.equip_location.as_ref()) {
                for part in location.split(',').filter(|p| !p.trim().is_empty()) {
                    let name = equip_location_name(part);
                    if !locations.contains(&name) {
                        locations.push(name);
                    }
                }
            }
        }
        locations
    }

    pub fn preconditions(&self, d: &GameData) -> Vec<i32> {
        self.item_component(d)
            .and_then(|ic| ic.req_precondition.clone())
            .unwrap_or_default()
    }

    /// The level required to use the item, read from its precondition text
    /// (`"You must be Level 35 …"`)
    pub fn level_requirement(&self, d: &GameData) -> Option<i32> {
        self.preconditions(d).into_iter().find_map(|id| {
            let reason = d.locale.preconditions.get(&id)?.failure_reason.as_deref()?;
            let lower = reason.to_lowercase();
            let rest = &reason[lower.find("level ")? + "level ".len()..];
            let digits: String = rest.chars().take_while(char::is_ascii_digit).collect();
            digits.parse().ok()
        })
    }

    /// Skills on this object and its proxies
    pub fn skills(&self, d: &GameData) -> Vec<ObjectSkill> {
        let mut skills = vec![];
        for object in std::iter::once(*self).chain(self.proxies(d)) {
            for os in d.cdclient.object_skills.at_group_key(&object.0).unwrap_or_default() {
                if !skills.iter().any(|s: &ObjectSkill| s.object == object.0 && s.skill == os.skill_id) {
                    skills.push(ObjectSkill {
                        object: object.0,
                        skill: os.skill_id,
                        cast_on_type: os.cast_on_type,
                    });
                }
            }
        }
        skills
    }

    /// (armor, life, imagination) bonuses from equipping the item
    pub fn stat_bonuses(&self, d: &GameData) -> (i32, i32, i32) {
        let mut stats = (0, 0, 0);
        for skill in self.skills(d) {
            if let Some(sb) = d.cdclient.skill_behavior.at_key(&skill.skill) {
                stats.0 += sb.armor_bonus_ui.unwrap_or(0);
                stats.1 += sb.life_bonus_ui.unwrap_or(0);
                stats.2 += sb.im_bonus_ui.unwrap_or(0);
            }
        }
        stats
    }

    /// Missions/achievements that reward this item
    pub fn missions_rewarding(&self, d: &GameData) -> Vec<CdClientMissionsId> {
        d.index
            .missions_rewarding_item
            .get(&self.0)
            .map(|ids| ids.iter().copied().map(CdClientMissionsId).collect())
            .unwrap_or_default()
    }

    /// Objects with a vendor component that sell this item
    pub fn vendors(&self, d: &GameData) -> Vec<CdClientObjectsId> {
        let mut vendors = vec![];
        for lmi in self.loot_matrices(d) {
            for &comp in d.index.vendors_with_matrix.get(&lmi).map_or(&[][..], Vec::as_slice) {
                for &object in d.objects_with_component(VENDOR_COMPONENT, comp) {
                    if d.is_visible(object) && !vendors.contains(&CdClientObjectsId(object)) {
                        vendors.push(CdClientObjectsId(object));
                    }
                }
            }
        }
        vendors.sort();
        vendors
    }

    /// Loot tables containing this object
    pub fn loot_tables<'a>(&self, d: &'a GameData) -> &'a [i32] {
        d.index.loot_tables_with_item.get(&self.0).map_or(&[], Vec::as_slice)
    }

    /// Loot matrices that can drop this object
    pub fn loot_matrices(&self, d: &GameData) -> Vec<i32> {
        let mut lmis: Vec<i32> = self
            .loot_tables(d)
            .iter()
            .flat_map(|lti| d.index.loot_matrices_with_table.get(lti).into_iter().flatten().copied())
            .collect();
        lmis.sort_unstable();
        lmis.dedup();
        lmis
    }

    /// Item sets (kits) that include this item
    pub fn item_sets<'a>(&self, d: &'a GameData) -> &'a [i32] {
        d.index.item_sets_with_item.get(&self.0).map_or(&[], Vec::as_slice)
    }

    pub fn npc_missions<'a>(&self, d: &'a GameData) -> &'a [crate::data::NpcMission] {
        d.index.npc_missions.get(&self.0).map_or(&[], Vec::as_slice)
    }

    pub fn is_npc(&self, d: &GameData) -> bool {
        self.has_component(d, MISSION_OFFER_COMPONENT) || !self.npc_missions(d).is_empty()
    }

    pub fn is_vendor(&self, d: &GameData) -> bool {
        self.has_component(d, VENDOR_COMPONENT)
    }

    pub fn is_package(&self, d: &GameData) -> bool {
        self.has_component(d, PACKAGE_COMPONENT)
    }

    /// Loot matrix dropped when smashed
    pub fn smash_loot_matrix(&self, d: &GameData) -> Option<i32> {
        self.destructible_component(d)?.loot_matrix_index
    }

    /// Loot matrix given when opened
    pub fn package_loot_matrix(&self, d: &GameData) -> Option<i32> {
        let comp = self.component_id(d, PACKAGE_COMPONENT)?;
        Some(d.cdclient.package_component.at_key(&comp)?.loot_matrix_index)
    }

    /// Loot matrix of items sold
    pub fn vendor_loot_matrix(&self, d: &GameData) -> Option<i32> {
        let comp = self.component_id(d, VENDOR_COMPONENT)?;
        Some(d.cdclient.vendor_component.at_key(&comp)?.loot_matrix_index)
    }
}

// -------- //
// Missions //
// -------- //

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CdClientMissionsId(pub i32);

impl LUExplorer for CdClientMissionsId {
    const NAME: &'static str = "Mission";
    const ENDPOINT: &'static str = "missions";

    fn id(&self) -> i32 {
        self.0
    }

    fn name(&self, d: &GameData) -> Option<String> {
        d.locale.missions.get(&self.0)?.name.clone()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct MissionReward {
    pub item: i32,
    pub count: i32,
    pub repeat_count: Option<i32>,
}

impl CdClientMissionsId {
    pub fn fetch<'a>(&self, d: &'a GameData) -> Option<&'a CdClientMissions> {
        d.cdclient.missions.at_key(&self.0)
    }

    pub fn try_fetch<'a>(&self, d: &'a GameData) -> BotResult<&'a CdClientMissions> {
        self.fetch(d).ok_or_else(|| self.not_found(d))
    }

    /// `"Type > Subtype"`
    pub fn category_path(&self, d: &GameData) -> String {
        let Some(mission) = self.fetch(d) else {
            return String::new();
        };
        match &mission.defined_subtype {
            Some(subtype) => format!("{} > {}", mission.defined_type, subtype),
            None => mission.defined_type.clone(),
        }
    }

    /// `"Type > Subtype > Name"`
    pub fn full_path(&self, d: &GameData) -> String {
        let path = self.category_path(d);
        if path.is_empty() {
            self.req_name(d)
        } else {
            format!("{path} > {}", self.req_name(d))
        }
    }

    pub fn is_mission(&self, d: &GameData) -> bool {
        self.fetch(d).is_some_and(|m| m.is_mission)
    }

    /// What the player is told to do
    pub fn objective(&self, d: &GameData) -> Option<String> {
        let text = d.locale.mission_text.get(&self.0);
        text.and_then(|t| t.description.clone())
            .or_else(|| text.and_then(|t| t.in_progress.clone()))
            .or_else(|| {
                // achievements describe themselves through their task
                d.cdclient
                    .mission_tasks
                    .at_group_key(&self.0)
                    .unwrap_or_default()
                    .iter()
                    .find_map(|task| d.locale.mission_tasks.get(&task.uid)?.description.clone())
            })
            .or_else(|| text.and_then(|t| t.completion_succeed_tip.clone()))
    }

    pub fn rewards(&self, d: &GameData) -> Vec<MissionReward> {
        let Some(m) = self.fetch(d) else {
            return vec![];
        };
        [
            (m.reward_item1, m.reward_item1_count, m.reward_item1_repeatable, m.reward_item1_repeat_count),
            (m.reward_item2, m.reward_item2_count, m.reward_item2_repeatable, m.reward_item2_repeat_count),
            (m.reward_item3, m.reward_item3_count, m.reward_item3_repeatable, m.reward_item3_repeat_count),
            (m.reward_item4, m.reward_item4_count, m.reward_item4_repeatable, m.reward_item4_repeat_count),
        ]
        .into_iter()
        .filter(|(item, ..)| *item > 0)
        .map(|(item, count, repeatable, repeat_count)| MissionReward {
            item,
            count: count.max(1),
            repeat_count: (m.repeatable && repeatable > 0).then_some(repeat_count.max(1)),
        })
        .collect()
    }

    pub fn thumbnail(&self, d: &GameData) -> Option<String> {
        let mission = self.fetch(d)?;
        mission
            .mission_icon_id
            .and_then(|icon| icon_url(d, icon))
            .or_else(|| {
                d.cdclient
                    .mission_tasks
                    .at_group_key(&self.0)
                    .unwrap_or_default()
                    .iter()
                    .find_map(|task| {
                        task.icon_id
                            .and_then(|icon| icon_url(d, icon))
                            .or_else(|| icon_url(d, task.large_task_icon_id?))
                    })
            })
    }
}

// ------ //
// Skills //
// ------ //

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CdClientSkillBehaviorId(pub i32);

impl LUExplorer for CdClientSkillBehaviorId {
    const NAME: &'static str = "Skill";
    const ENDPOINT: &'static str = "skills";

    fn id(&self) -> i32 {
        self.0
    }

    fn name(&self, d: &GameData) -> Option<String> {
        d.locale.skill_behavior.get(&self.0)?.name.clone()
    }
}

impl CdClientSkillBehaviorId {
    pub fn fetch<'a>(&self, d: &'a GameData) -> Option<&'a CdClientSkillBehavior> {
        d.cdclient.skill_behavior.at_key(&self.0)
    }

    pub fn try_fetch<'a>(&self, d: &'a GameData) -> BotResult<&'a CdClientSkillBehavior> {
        self.fetch(d).ok_or_else(|| self.not_found(d))
    }

    pub fn thumbnail(&self, d: &GameData) -> Option<String> {
        icon_url(d, self.fetch(d)?.skill_icon?)
    }

    pub fn description<'a>(&self, d: &'a GameData) -> Option<&'a SkillBehaviorDescriptionUI> {
        d.locale.skill_behavior.get(&self.0)?.description_ui.as_ref()
    }

    /// The description as one block of text
    pub fn description_text(&self, d: &GameData) -> Option<String> {
        let desc = self.description(d)?;
        let mut parts = vec![];
        if let Some(remainder) = &desc.remainder {
            parts.push(remainder.clone());
        }
        for (name, value) in desc.segments.iter().flatten() {
            parts.push(format!("**{}:** {value}", split_camel_case(name)));
        }
        (!parts.is_empty()).then(|| parts.join("\n"))
    }

    pub fn objects<'a>(&self, d: &'a GameData) -> &'a [i32] {
        d.index.objects_with_skill.get(&self.0).map_or(&[], Vec::as_slice)
    }
}

/// `"DamageCombo"` -> `"Damage Combo"`
pub fn split_camel_case(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 4);
    let mut prev: Option<char> = None;
    for ch in text.chars() {
        if ch.is_uppercase() && prev.is_some_and(|p| p.is_lowercase() || p.is_ascii_digit()) {
            out.push(' ');
        }
        out.push(ch);
        prev = Some(ch);
    }
    out
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CooldownGroupId(pub i32);

impl LUExplorer for CooldownGroupId {
    const NAME: &'static str = "Cooldown Group";
    const ENDPOINT: &'static str = "skills/cooldowngroups";

    fn id(&self) -> i32 {
        self.0
    }
}

impl CooldownGroupId {
    pub fn skills<'a>(&self, d: &'a GameData) -> &'a [i32] {
        d.index
            .skills_in_cooldown_group
            .get(&self.0)
            .map_or(&[], Vec::as_slice)
    }
}

// ---------- //
// Activities //
// ---------- //

/// An activity, identified by `ActivityRewards.objectTemplate` / `Activities.ActivityID`
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CdClientActivityId(pub i32);

impl LUExplorer for CdClientActivityId {
    const NAME: &'static str = "Activity";
    const ENDPOINT: &'static str = "activities";

    fn id(&self) -> i32 {
        self.0
    }

    /// The locale name, otherwise the common start of its reward descriptions
    /// (`"Survival 1"`, `"Survival 2"` -> `"Survival"`)
    fn name(&self, d: &GameData) -> Option<String> {
        if let Some(name) = d.locale.activities.get(&self.0).and_then(|a| a.activity_name.clone()) {
            return Some(name);
        }
        let rewards = self.rewards(d);
        let first = rewards.first()?.description.trim();
        if rewards.len() == 1 {
            return (!first.is_empty()).then(|| first.to_string());
        }
        let common = rewards.iter().skip(1).fold(first.to_string(), |acc, r| {
            acc.chars()
                .zip(r.description.chars())
                .take_while(|(a, b)| a == b)
                .map(|(a, _)| a)
                .collect()
        });
        let name = common
            .trim_end_matches(|c: char| c.is_ascii_digit() || c.is_whitespace() || c == '-' || c == '(')
            .trim();
        if name.chars().count() >= 3 {
            Some(name.to_string())
        } else {
            (!first.is_empty()).then(|| first.to_string())
        }
    }
}

impl CdClientActivityId {
    pub fn rewards<'a>(&self, d: &'a GameData) -> &'a [CdClientActivityRewards] {
        d.cdclient.activity_rewards.at_group_key(&self.0).unwrap_or_default()
    }
}

// ----------- //
// Loot Tables //
// ----------- //

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CdClientLootTableId(pub i32);

impl LUExplorer for CdClientLootTableId {
    const NAME: &'static str = "Loot Table";
    const ENDPOINT: &'static str = "objects/loot/table";

    fn id(&self) -> i32 {
        self.0
    }

    fn name(&self, d: &GameData) -> Option<String> {
        d.loot_table_names.get(&self.0).cloned()
    }
}

impl CdClientLootTableId {
    pub fn exists(&self, d: &GameData) -> bool {
        d.cdclient.loot_table.at_group_key(&self.0).is_some()
    }

    /// Items in the table, in the table's sort order
    pub fn items(&self, d: &GameData) -> Vec<CdClientObjectsId> {
        let mut rows: Vec<_> = d
            .cdclient
            .loot_table
            .at_group_key(&self.0)
            .unwrap_or_default()
            .iter()
            .collect();
        rows.sort_by_key(|lt| (lt.sort_priority, lt.id));
        let mut items: Vec<CdClientObjectsId> = vec![];
        for lt in rows {
            let item = CdClientObjectsId(lt.itemid);
            if !items.contains(&item) && d.is_visible(item.0) {
                items.push(item);
            }
        }
        items
    }

    /// Number of items of a rarity (`None` = every item)
    pub fn count(&self, d: &GameData, rarity: Option<i32>) -> usize {
        d.index
            .loot_table_rarity_counts
            .get(&(self.0, rarity))
            .copied()
            .unwrap_or(0)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CdClientLootMatrixId(pub i32);

impl LUExplorer for CdClientLootMatrixId {
    const NAME: &'static str = "Loot Matrix";
    const ENDPOINT: &'static str = "objects/loot/matrix";

    fn id(&self) -> i32 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn camel_case() {
        assert_eq!(split_camel_case("DamageCombo"), "Damage Combo");
        assert_eq!(split_camel_case("Description"), "Description");
        assert_eq!(split_camel_case("ChargeUp2Hit"), "Charge Up2 Hit");
    }

    #[test]
    fn markdown_escaping() {
        assert_eq!(escape_markdown("a_b*c [d]"), r"a\_b\*c \[d\]");
    }

    #[test]
    fn equip_locations() {
        assert_eq!(equip_location_name("special_r"), "Right Hand");
        assert_eq!(equip_location_name("clavicle"), "Armor");
    }
}
