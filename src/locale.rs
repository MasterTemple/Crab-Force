/// I only need the phrases in en_US so I will not deal with all the config parts
use std::{collections::BTreeMap, fs, path::Path};

use once_cell::sync::Lazy;
use regex::Regex;

/// trait name says it all
/// pass a closure to modify an object at an index
/// if the object is not there, it will be created by default and then you will be modifying that
pub trait ModifyExistingOrInsertDefaultAndModify<K, V: Default> {
    fn modify(&mut self, key: K, func: impl Fn(&mut V));
}

impl<K: Ord + Clone, V: std::fmt::Debug + Default> ModifyExistingOrInsertDefaultAndModify<K, V>
    for BTreeMap<K, V>
{
    fn modify(&mut self, key: K, func: impl Fn(&mut V)) {
        if !self.contains_key(&key) {
            _ = self.insert(key.clone(), V::default());
        }
        let this = self
            .get_mut(&key)
            .expect("Just inserted if it wasn't already there");
        func(this);
    }
}

#[derive(Debug, Default)]
pub struct LocaleXML {
    pub locales: BTreeMap<String, LocaleTranslation>,
}

static PHRASE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"\s*<phrase id="([^"]+)">(\s*<translation locale="[^"]+">.*</translation>\s*)+\s*</phrase>\s*"#).unwrap()
});

static TRANSLATION_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"<translation locale="([^"]+)">(.*)</translation>"#).unwrap());

impl LocaleXML {
    pub fn load_xml(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(path)?;
        let mut xml = LocaleXML::default();
        for matched_phase in PHRASE_REGEX.captures_iter(&contents) {
            let key = matched_phase.get(1).unwrap().as_str();
            let translations = matched_phase.get(0).unwrap().as_str();

            for matched_translation in TRANSLATION_REGEX.captures_iter(translations) {
                let locale = matched_translation.get(1).unwrap().as_str();
                let content = matched_translation.get(2).unwrap().as_str();

                // get translation object
                if !xml.locales.contains_key(locale) {
                    _ = xml
                        .locales
                        .insert(locale.to_string(), LocaleTranslation::default());
                }
                let translation = xml.locales.get_mut(locale).expect("Key already added");
                // match phrase key
                if let Some(rem) = key.starts_with_remainder(LocaleActivities::PREFIX) {
                    if let Some(id) = parse_id_from_remainder(rem, "_ActivityName") {
                        translation
                            .activities
                            .modify(id, |it| it.activity_name = Some(content.to_string()));
                    }
                } else if let Some(rem) = key.starts_with_remainder(LocaleItemSets::PREFIX) {
                    if let Some(id) = parse_id_from_remainder(rem, "_kitName") {
                        translation
                            .item_sets
                            .modify(id, |it| it.kit_name = Some(content.to_string()));
                    }
                } else if let Some(rem) = key.starts_with_remainder(LocaleMissionEmail::PREFIX) {
                    if let Some(id) = parse_id_from_remainder(rem, "_announceText") {
                        translation
                            .mission_email
                            .modify(id, |it| it.announce_text = Some(content.to_string()));
                    } else if let Some(id) = parse_id_from_remainder(rem, "_bodyText") {
                        translation
                            .mission_email
                            .modify(id, |it| it.body_text = Some(content.to_string()));
                    } else if let Some(id) = parse_id_from_remainder(rem, "_senderName") {
                        translation
                            .mission_email
                            .modify(id, |it| it.sender_name = Some(content.to_string()));
                    } else if let Some(id) = parse_id_from_remainder(rem, "_subjectText") {
                        translation
                            .mission_email
                            .modify(id, |it| it.subject_text = Some(content.to_string()));
                    }
                } else if let Some(rem) = key.starts_with_remainder(LocaleMissionTasks::PREFIX) {
                    if let Some(id) = parse_id_from_remainder(rem, "_description") {
                        translation
                            .mission_tasks
                            .modify(id, |it| it.description = Some(content.to_string()));
                    }
                } else if let Some(rem) = key.starts_with_remainder(LocaleMissionText::PREFIX) {
                    if let Some(id) = parse_id_from_remainder(rem, "_accept_chat_bubble") {
                        translation
                            .mission_text
                            .modify(id, |it| it.accept_chat_bubble = Some(content.to_string()));
                    } else if let Some(id) = parse_id_from_remainder(rem, "_chat_state_1") {
                        translation
                            .mission_text
                            .modify(id, |it| it.chat_state_1 = Some(content.to_string()));
                    } else if let Some(id) = parse_id_from_remainder(rem, "_chat_state_2") {
                        translation
                            .mission_text
                            .modify(id, |it| it.chat_state_2 = Some(content.to_string()));
                    } else if let Some(id) = parse_id_from_remainder(rem, "_chat_state_3_turnin") {
                        translation
                            .mission_text
                            .modify(id, |it| it.chat_state_3_turnin = Some(content.to_string()));
                    } else if let Some(id) = parse_id_from_remainder(rem, "_completion_succeed_tip")
                    {
                        translation.mission_text.modify(id, |it| {
                            it.completion_succeed_tip = Some(content.to_string())
                        });
                    } else if let Some(id) = parse_id_from_remainder(rem, "_in_progress") {
                        translation
                            .mission_text
                            .modify(id, |it| it.in_progress = Some(content.to_string()));
                    } else if let Some(id) = parse_id_from_remainder(rem, "_offer") {
                        translation
                            .mission_text
                            .modify(id, |it| it.offer = Some(content.to_string()));
                    } else if let Some(id) = parse_id_from_remainder(rem, "_ready_to_complete") {
                        translation
                            .mission_text
                            .modify(id, |it| it.ready_to_complete = Some(content.to_string()));
                    } else if let Some(id) = parse_id_from_remainder(rem, "_description") {
                        translation
                            .mission_text
                            .modify(id, |it| it.description = Some(content.to_string()));
                    } else if let Some(id) = parse_id_from_remainder(rem, "_chat_state_3") {
                        translation
                            .mission_text
                            .modify(id, |it| it.chat_state_3 = Some(content.to_string()));
                    } else if let Some(id) = parse_id_from_remainder(rem, "_chat_state_4") {
                        translation
                            .mission_text
                            .modify(id, |it| it.chat_state_4 = Some(content.to_string()));
                    } else if let Some(id) = parse_id_from_remainder(rem, "_chat_state_4_turnin") {
                        translation
                            .mission_text
                            .modify(id, |it| it.chat_state_4_turnin = Some(content.to_string()));
                    } else if let Some(id) = parse_id_from_remainder(rem, "_offer_repeatable") {
                        translation
                            .mission_text
                            .modify(id, |it| it.offer_repeatable = Some(content.to_string()));
                    }
                } else if let Some(rem) = key.starts_with_remainder(LocaleMissions::PREFIX) {
                    if let Some(id) = parse_id_from_remainder(rem, "_name") {
                        translation
                            .missions
                            .modify(id, |it| it.name = Some(content.to_string()));
                    }
                } else if let Some(rem) = key.starts_with_remainder(LocaleObjects::PREFIX) {
                    if let Some(id) = parse_id_from_remainder(rem, "_name") {
                        translation
                            .objects
                            .modify(id, |it| it.name = Some(content.to_string()));
                    } else if let Some(id) = parse_id_from_remainder(rem, "_description") {
                        translation
                            .objects
                            .modify(id, |it| it.description = Some(content.to_string()));
                    }
                } else if let Some(rem) = key.starts_with_remainder(LocalePreconditions::PREFIX) {
                    if let Some(id) = parse_id_from_remainder(rem, "_FailureReason") {
                        translation
                            .preconditions
                            .modify(id, |it| it.failure_reason = Some(content.to_string()));
                    }
                } else if let Some(rem) = key.starts_with_remainder(LocaleSkillBehavior::PREFIX) {
                    if let Some(id) = parse_id_from_remainder(rem, "_name") {
                        translation
                            .skill_behavior
                            .modify(id, |it| it.name = Some(content.to_string()));
                    } else if let Some(id) = parse_id_from_remainder(rem, "_descriptionUI") {
                        translation.skill_behavior.modify(id, |it| {
                            it.description_ui = Some(SkillBehaviorDescriptionUI::new(content))
                        });
                    }
                } else {
                    _ = translation
                        .other
                        .insert(key.to_string(), content.to_string());
                }
            }
        }
        Ok(xml)
    }
}

type LocaleMap<T> = BTreeMap<i32, T>;

#[derive(Debug, Default)]
pub struct LocaleTranslation {
    pub activities: LocaleMap<LocaleActivities>,
    pub item_sets: LocaleMap<LocaleItemSets>,
    pub mission_email: LocaleMap<LocaleMissionEmail>,
    pub mission_tasks: LocaleMap<LocaleMissionTasks>,
    pub mission_text: LocaleMap<LocaleMissionText>,
    pub missions: LocaleMap<LocaleMissions>,
    pub objects: LocaleMap<LocaleObjects>,
    pub preconditions: LocaleMap<LocalePreconditions>,
    pub skill_behavior: LocaleMap<LocaleSkillBehavior>,
    // when the phrase is not related to a particular id
    pub other: BTreeMap<String, String>,
}

trait StartsWithStripped {
    /// check if it starts with substr and then returns remaining
    fn starts_with_remainder(&self, prefix: &str) -> Option<&str>;
}

impl StartsWithStripped for &str {
    fn starts_with_remainder(&self, prefix: &str) -> Option<&str> {
        self.starts_with(prefix).then(|| &self[prefix.len()..])
    }
}

/// check if ends with [`after`] and returns parsed id
fn parse_id_from_remainder(key: &str, after: &str) -> Option<i32> {
    let start = 0;
    if !key.ends_with(after) {
        return None;
    }
    let end = key.len() - after.len();
    // dbg!(&key, &key[start..end]);
    // panic!("yay");
    key[start..end].parse().ok()
}

pub trait IdentifiedGroup {
    const PREFIX: &'static str;
}

/// Locale Phrases:
/// - Activities_{ID}_ActivityName
#[derive(Clone, Debug, Default)]
pub struct LocaleActivities {
    pub activity_name: Option<String>,
}
impl IdentifiedGroup for LocaleActivities {
    const PREFIX: &'static str = "Activities_";
}

/// Locale Phrases:
/// - ItemSets_{ID}_kitName
#[derive(Clone, Debug, Default)]
pub struct LocaleItemSets {
    pub kit_name: Option<String>,
}
impl IdentifiedGroup for LocaleItemSets {
    const PREFIX: &'static str = "ItemSets_";
}

/// Locale Phrases:
/// - MissionEmail_{ID}_announceText
/// - MissionEmail_{ID}_bodyText
/// - MissionEmail_{ID}_senderName
/// - MissionEmail_{ID}_subjectText
#[derive(Clone, Debug, Default)]
pub struct LocaleMissionEmail {
    pub announce_text: Option<String>,
    pub body_text: Option<String>,
    pub sender_name: Option<String>,
    pub subject_text: Option<String>,
}
impl IdentifiedGroup for LocaleMissionEmail {
    const PREFIX: &'static str = "MissionEmail_";
}

/// Locale Phrases:
/// - MissionTasks_{ID}_description
#[derive(Clone, Debug, Default)]
pub struct LocaleMissionTasks {
    pub description: Option<String>,
}
impl IdentifiedGroup for LocaleMissionTasks {
    const PREFIX: &'static str = "MissionTasks_";
}

/// Locale Phrases:
/// - MissionText_{ID}_accept_chat_bubble
/// - MissionText_{ID}_chat_state_1
/// - MissionText_{ID}_chat_state_2
/// - MissionText_{ID}_chat_state_3_turnin
/// - MissionText_{ID}_completion_succeed_tip
/// - MissionText_{ID}_in_progress
/// - MissionText_{ID}_offer
/// - MissionText_{ID}_ready_to_complete
/// - MissionText_{ID}_description
/// - MissionText_{ID}_chat_state_3
/// - MissionText_{ID}_chat_state_4
/// - MissionText_{ID}_chat_state_4_turnin
/// - MissionText_{ID}_offer_repeatable
#[derive(Clone, Debug, Default)]
pub struct LocaleMissionText {
    pub accept_chat_bubble: Option<String>,
    pub chat_state_1: Option<String>,
    pub chat_state_2: Option<String>,
    pub chat_state_3_turnin: Option<String>,
    pub completion_succeed_tip: Option<String>,
    pub in_progress: Option<String>,
    pub offer: Option<String>,
    pub ready_to_complete: Option<String>,
    pub description: Option<String>,
    pub chat_state_3: Option<String>,
    pub chat_state_4: Option<String>,
    pub chat_state_4_turnin: Option<String>,
    pub offer_repeatable: Option<String>,
}
impl IdentifiedGroup for LocaleMissionText {
    const PREFIX: &'static str = "MissionText_";
}

/// Locale Phrases:
/// - Missions_{ID}_name
#[derive(Clone, Debug, Default)]
pub struct LocaleMissions {
    pub name: Option<String>,
}
impl IdentifiedGroup for LocaleMissions {
    const PREFIX: &'static str = "Missions_";
}

/// Locale Phrases:
/// - Objects_{ID}_name
/// - Objects_{ID}_description
#[derive(Clone, Debug, Default)]
pub struct LocaleObjects {
    pub name: Option<String>,
    pub description: Option<String>,
}
impl IdentifiedGroup for LocaleObjects {
    const PREFIX: &'static str = "Objects_";
}

/// Locale Phrases:
/// - Preconditions_{ID}_FailureReason
#[derive(Clone, Debug, Default)]
pub struct LocalePreconditions {
    pub failure_reason: Option<String>,
}
impl IdentifiedGroup for LocalePreconditions {
    const PREFIX: &'static str = "Preconditions_";
}

/// Locale Phrases:
/// - SkillBehavior_{ID}_name
/// - SkillBehavior_{ID}_descriptionUI
#[derive(Clone, Debug, Default)]
pub struct LocaleSkillBehavior {
    pub name: Option<String>,
    // pub description_ui: Option<String>,
    pub description_ui: Option<SkillBehaviorDescriptionUI>,
}
impl IdentifiedGroup for LocaleSkillBehavior {
    const PREFIX: &'static str = "SkillBehavior_";
}

/// - SkillBehavior_{ID}_descriptionUI
#[derive(Clone, Debug, Default)]
pub struct SkillBehaviorDescriptionUI {
    /// the whole text
    // text: Option<String>,
    /// the remainder
    remainder: Option<String>,
    /// segments
    segments: Option<BTreeMap<String, String>>,
}
impl SkillBehaviorDescriptionUI {
    pub fn new(text: &str) -> Self {
        let mut is_first = true;
        let mut segments = BTreeMap::new();
        let mut remainder = None;
        for cap in DESCRIPTION_UI_REGEX.captures_iter(text) {
            if is_first {
                is_first = false;
                let entire_capture = cap.get(0).unwrap();
                let start = text[..entire_capture.start()].trim();
                if start.len() > 0 {
                    remainder = Some(start.to_string());
                }
            }
            let key = cap.get(1).unwrap().as_str().to_string();
            let value = cap.get(2).unwrap().as_str().to_string();
            segments.insert(key, value);
        }
        let segments = if segments.len() != 0 {
            Some(segments)
        } else {
            None
        };
        Self {
            remainder,
            segments,
        }
    }
}

/**
Given
```xml
<translation locale="en_US">%(DamageCombo) 3+4+4 %(ChargeUp)Does 12 damage to enemies</translation>
```
Which would have already been parsed to
```no_run
"%(DamageCombo) 3+4+4 %(ChargeUp)Does 12 damage to enemies"
```
Matches
```no_run
Some(Captures({
    0: Some("%(DamageCombo) 3+4+4 "),
    1: Some("DamageCombo"),
    2: Some("3+4+4"),
})),
Some(Captures({
    0: Some("%(ChargeUp)Does 12 damage to enemies\n"),
    1: Some("ChargeUp"),
    2: Some("Does 12 damage to enemies"),
})),
```
*/
static DESCRIPTION_UI_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"%\(([^\)]+)\)\s*([^%]*[^\s%])\s*"#).unwrap());
