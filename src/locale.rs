//! Parsing of the client's `locale.xml`
use std::{collections::BTreeMap, fs, path::Path, sync::LazyLock as Lazy};

use regex::Regex;

use crate::error::LocaleLoadError;

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

/// Reads the translations for one locale out of `locale.xml`
///
/// The file is machine generated with a very regular shape, so a small scanner is used instead
/// of a full XML parser:
/// ```xml
/// <phrase id="Objects_1727_name">
///     <translation locale="en_US">Bat Lord Cape</translation>
/// </phrase>
/// ```
/// Entities (`&amp;`, `&lt;`, `&#39;`, …) are decoded and in-game markup such as
/// `<font color='#FF7F00'>` is removed.
pub fn load_locale(path: &Path, locale: &str) -> Result<LocaleTranslation, LocaleLoadError> {
    if !path.is_file() {
        return Err(LocaleLoadError::NotFound(path.to_path_buf()));
    }
    let contents = fs::read_to_string(path).map_err(|source| LocaleLoadError::Read {
        path: path.to_path_buf(),
        source,
    })?;
    parse_locale(&contents, locale)
}

pub fn parse_locale(contents: &str, locale: &str) -> Result<LocaleTranslation, LocaleLoadError> {
    let mut translation = LocaleTranslation::default();
    let mut seen_locales: Vec<&str> = vec![];
    let mut found = false;

    for (key, phrase_body) in phrases(contents) {
        for (phrase_locale, raw) in translations(phrase_body) {
            if !seen_locales.contains(&phrase_locale) {
                seen_locales.push(phrase_locale);
            }
            if phrase_locale != locale {
                continue;
            }
            found = true;
            let content = clean_text(raw);
            if content.is_empty() {
                continue;
            }
            translation.insert(key, content);
        }
    }

    if !found {
        return Err(LocaleLoadError::MissingLocale {
            locale: locale.to_string(),
            available: if seen_locales.is_empty() {
                String::from("none; is this really a locale.xml?")
            } else {
                seen_locales.join(", ")
            },
        });
    }
    Ok(translation)
}

/// Yields `(phrase id, inner xml)` for every `<phrase id="…">…</phrase>`
fn phrases(contents: &str) -> impl Iterator<Item = (&str, &str)> {
    const OPEN: &str = "<phrase id=\"";
    const CLOSE: &str = "</phrase>";
    let mut rest = contents;
    std::iter::from_fn(move || loop {
        let start = rest.find(OPEN)? + OPEN.len();
        let after = &rest[start..];
        let id_end = after.find('"')?;
        let id = &after[..id_end];
        let after_id = &after[id_end..];
        let body_start = after_id.find('>')? + 1;
        // `<phrase id="x"/>` has no translations
        if after_id[..body_start].ends_with("/>") {
            rest = &after_id[body_start..];
            continue;
        }
        let body = &after_id[body_start..];
        let body_end = body.find(CLOSE).unwrap_or(body.len());
        rest = &body[body_end..];
        return Some((id, &body[..body_end]));
    })
}

/// Yields `(locale, raw text)` for every `<translation locale="…">…</translation>`
fn translations(phrase: &str) -> impl Iterator<Item = (&str, &str)> {
    const OPEN: &str = "<translation locale=\"";
    const CLOSE: &str = "</translation>";
    let mut rest = phrase;
    std::iter::from_fn(move || loop {
        let start = rest.find(OPEN)? + OPEN.len();
        let after = &rest[start..];
        let locale_end = after.find('"')?;
        let locale = &after[..locale_end];
        let after_locale = &after[locale_end..];
        let text_start = after_locale.find('>')? + 1;
        if after_locale[..text_start].ends_with("/>") {
            rest = &after_locale[text_start..];
            continue;
        }
        let text = &after_locale[text_start..];
        let text_end = text.find(CLOSE).unwrap_or(text.len());
        rest = &text[text_end..];
        return Some((locale, &text[..text_end]));
    })
}

/// Decodes XML entities, then strips in-game markup tags and surrounding whitespace
pub fn clean_text(raw: &str) -> String {
    let decoded = decode_entities(raw);
    MARKUP_REGEX.replace_all(&decoded, "").trim().to_string()
}

fn decode_entities(raw: &str) -> String {
    if !raw.contains('&') {
        return raw.to_string();
    }
    let mut out = String::with_capacity(raw.len());
    let mut rest = raw;
    while let Some(amp) = rest.find('&') {
        out.push_str(&rest[..amp]);
        let after = &rest[amp..];
        let decoded = after.find(';').filter(|&end| end <= 10).and_then(|end| {
            let entity = &after[1..end];
            let ch = match entity {
                "amp" => Some('&'),
                "lt" => Some('<'),
                "gt" => Some('>'),
                "quot" => Some('"'),
                "apos" => Some('\''),
                "nbsp" => Some(' '),
                _ => entity
                    .strip_prefix("#x")
                    .or_else(|| entity.strip_prefix("#X"))
                    .and_then(|hex| u32::from_str_radix(hex, 16).ok())
                    .or_else(|| entity.strip_prefix('#').and_then(|dec| dec.parse().ok()))
                    .and_then(char::from_u32),
            };
            ch.map(|ch| (ch, end))
        });
        match decoded {
            Some((ch, end)) => {
                out.push(ch);
                rest = &after[end + 1..];
            }
            None => {
                out.push('&');
                rest = &after[1..];
            }
        }
    }
    out.push_str(rest);
    out
}

/// In-game rich text such as `<font color='#FF7F00'>` and `</font>`
static MARKUP_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"</?[A-Za-z][^<>]*>").expect("valid regex"));

impl LocaleTranslation {
    /// Files a phrase under the matching id-based table, or [`LocaleTranslation::other`]
    fn insert(&mut self, key: &str, content: String) {
        let translation = self;
        let content = content.as_str();
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
    pub remainder: Option<String>,
    /// segments
    pub segments: Option<BTreeMap<String, String>>,
}
impl SkillBehaviorDescriptionUI {
    /// Splits `"Intro %(DamageCombo) 3+4+4 %(ChargeUp)Does 12 damage"` into
    /// - remainder: `"Intro"`
    /// - segments: `DamageCombo => "3+4+4"`, `ChargeUp => "Does 12 damage"`
    ///
    /// Text without any `%(…)` markers becomes the remainder, and a literal `%` (e.g. `50%`)
    /// is kept as part of the text.
    pub fn new(text: &str) -> Self {
        let markers: Vec<_> = SEGMENT_MARKER_REGEX.captures_iter(text).collect();
        let first_start = markers
            .first()
            .and_then(|cap| cap.get(0))
            .map_or(text.len(), |m| m.start());
        let remainder = Some(text[..first_start].trim().to_string()).filter(|r| !r.is_empty());

        let mut segments = BTreeMap::new();
        for (idx, cap) in markers.iter().enumerate() {
            let (Some(whole), Some(key)) = (cap.get(0), cap.get(1)) else {
                continue;
            };
            let end = markers
                .get(idx + 1)
                .and_then(|next| next.get(0))
                .map_or(text.len(), |m| m.start());
            let value = text[whole.end()..end].trim();
            if !value.is_empty() {
                segments.insert(key.as_str().trim().to_string(), value.to_string());
            }
        }

        Self {
            remainder,
            segments: (!segments.is_empty()).then_some(segments),
        }
    }
}

/// Matches the `%(SegmentName)` markers in a `SkillBehavior_{ID}_descriptionUI`
static SEGMENT_MARKER_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"%\(([^)]+)\)").expect("valid regex"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn description_segments() {
        let desc = SkillBehaviorDescriptionUI::new(
            "%(DamageCombo) 3+4+4 %(ChargeUp)Does 12 damage to enemies\n",
        );
        assert_eq!(desc.remainder, None);
        let segments = desc.segments.unwrap();
        assert_eq!(segments["DamageCombo"], "3+4+4");
        assert_eq!(segments["ChargeUp"], "Does 12 damage to enemies");
    }

    #[test]
    fn plain_description_is_kept() {
        let desc = SkillBehaviorDescriptionUI::new("Increases speed by 50% for 10 seconds");
        assert_eq!(desc.remainder.as_deref(), Some("Increases speed by 50% for 10 seconds"));
        assert!(desc.segments.is_none());
    }

    #[test]
    fn entities_and_markup() {
        assert_eq!(
            clean_text("Press &lt;font color=&apos;#FF7F00&apos;&gt;5&lt;/font&gt; now &amp; then &#39;x&#x27;"),
            "Press 5 now & then 'x'"
        );
        assert_eq!(clean_text("Fish &chips & more"), "Fish &chips & more");
    }

    #[test]
    fn parses_phrases() {
        let xml = r#"<localization><phrases count="3">
            <phrase id="Objects_1727_name">
                <translation locale="en_US">Bat Lord
Cape</translation>
                <translation locale="de_DE">Fledermaus</translation>
            </phrase>
            <phrase id="Missions_5_name"><translation locale="en_US">Fish &amp; Chips</translation></phrase>
            <phrase id="Empty"/>
            <phrase id="UI_THING"><translation locale="en_US"/></phrase>
        </phrases></localization>"#;
        let t = parse_locale(xml, "en_US").unwrap();
        assert_eq!(t.objects[&1727].name.as_deref(), Some("Bat Lord\nCape"));
        assert_eq!(t.missions[&5].name.as_deref(), Some("Fish & Chips"));
        assert!(matches!(
            parse_locale(xml, "fr_FR"),
            Err(LocaleLoadError::MissingLocale { .. })
        ));
    }
}
