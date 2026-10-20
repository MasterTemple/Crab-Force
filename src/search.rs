//! Ranked, typo-tolerant searching used by autocomplete, `/search`, and for resolving typed names
//!
//! Matches are tiered (a better tier always wins), then sorted by a small quality bonus, name
//! length, and id:
//! 1. id equals the query
//! 2. name equals the query
//! 3. name starts with the query
//! 4. a word in the name starts with the query
//! 5. name contains the query
//! 6. every query word starts a word in the name/aliases (any order)
//! 7. every query word appears somewhere in the name/aliases
//! 8. every query word is in the name or is a near typo of a word
//! 9. id starts with the query (when typing a number)

use std::collections::HashMap;
use std::fmt::Display;

use serenity::all::AutocompleteChoice;

use crate::embed::truncate;
use crate::error::{BotError, BotResult};

/// Discord allows at most 25 autocomplete choices
pub const AUTOCOMPLETE_LIMIT: usize = 25;
/// Discord's limit for an autocomplete choice name
const CHOICE_NAME_LIMIT: usize = 100;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SearchCategory {
    Object,
    Item,
    Brick,
    Enemy,
    Smashable,
    Package,
    Vendor,
    Npc,
    Mission,
    Achievement,
    Skill,
    Activity,
    LootTable,
    CooldownGroup,
}

impl SearchCategory {
    pub const ALL: [SearchCategory; 14] = [
        SearchCategory::Object,
        SearchCategory::Item,
        SearchCategory::Brick,
        SearchCategory::Enemy,
        SearchCategory::Smashable,
        SearchCategory::Package,
        SearchCategory::Vendor,
        SearchCategory::Npc,
        SearchCategory::Mission,
        SearchCategory::Achievement,
        SearchCategory::Skill,
        SearchCategory::Activity,
        SearchCategory::LootTable,
        SearchCategory::CooldownGroup,
    ];

    /// Singular, human readable
    pub fn label(&self) -> &'static str {
        match self {
            SearchCategory::Object => "object",
            SearchCategory::Item => "item",
            SearchCategory::Brick => "brick",
            SearchCategory::Enemy => "enemy",
            SearchCategory::Smashable => "smashable",
            SearchCategory::Package => "package",
            SearchCategory::Vendor => "vendor",
            SearchCategory::Npc => "NPC",
            SearchCategory::Mission => "mission",
            SearchCategory::Achievement => "achievement",
            SearchCategory::Skill => "skill",
            SearchCategory::Activity => "activity",
            SearchCategory::LootTable => "loot table",
            SearchCategory::CooldownGroup => "cooldown group",
        }
    }

    /// Value used for `/search`'s category choice
    pub fn key(&self) -> &'static str {
        match self {
            SearchCategory::Object => "object",
            SearchCategory::Item => "item",
            SearchCategory::Brick => "brick",
            SearchCategory::Enemy => "enemy",
            SearchCategory::Smashable => "smashable",
            SearchCategory::Package => "package",
            SearchCategory::Vendor => "vendor",
            SearchCategory::Npc => "npc",
            SearchCategory::Mission => "mission",
            SearchCategory::Achievement => "achievement",
            SearchCategory::Skill => "skill",
            SearchCategory::Activity => "activity",
            SearchCategory::LootTable => "loottable",
            SearchCategory::CooldownGroup => "cooldowngroup",
        }
    }

    pub fn from_key(key: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|c| c.key() == key)
    }
}

/// Lowercases, drops apostrophes (`Overlord's` -> `overlords`), and turns every other
/// non-alphanumeric character into a single space
pub fn normalize(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut pending_space = false;
    for ch in text.chars() {
        if matches!(ch, '\'' | '’' | '`') {
            continue;
        }
        if ch.is_alphanumeric() {
            if pending_space && !out.is_empty() {
                out.push(' ');
            }
            pending_space = false;
            out.extend(ch.to_lowercase());
        } else {
            pending_space = true;
        }
    }
    out
}

#[derive(Clone, Debug)]
pub struct SearchEntry {
    pub id: i32,
    /// What is shown to users
    pub name: String,
    /// Extra context shown after the name in `/search` (e.g. object type)
    pub detail: Option<String>,
    norm: String,
    /// `norm` without spaces, so `"batlord"` finds `"bat lord"`
    compact: String,
    /// Normalized alternate names (internal name, mission category, …)
    aliases: Vec<String>,
    /// Small ranking boost within a tier (e.g. has an official in-game name)
    bonus: i32,
}

impl SearchEntry {
    pub fn new(id: i32, name: impl Into<String>) -> Self {
        let name = name.into();
        let norm = normalize(&name);
        Self {
            id,
            compact: norm.replace(' ', ""),
            norm,
            name,
            detail: None,
            aliases: vec![],
            bonus: 0,
        }
    }

    pub fn alias(mut self, alias: impl AsRef<str>) -> Self {
        let alias = normalize(alias.as_ref());
        if !alias.is_empty() && alias != self.norm && !self.aliases.contains(&alias) {
            self.aliases.push(alias);
        }
        self
    }

    pub fn detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }

    pub fn bonus(mut self, bonus: i32) -> Self {
        self.bonus = bonus;
        self
    }

    /// `Name [id]`, guaranteed to fit in an autocomplete choice
    pub fn label(&self) -> String {
        let suffix = format!(" [{}]", self.id);
        let name = truncate(&self.name, CHOICE_NAME_LIMIT - suffix.chars().count());
        format!("{name}{suffix}")
    }

    fn texts(&self) -> impl Iterator<Item = &str> {
        std::iter::once(self.norm.as_str()).chain(self.aliases.iter().map(String::as_str))
    }

    fn words(&self) -> impl Iterator<Item = &str> {
        self.texts().flat_map(|t| t.split(' '))
    }
}

#[derive(Debug)]
pub struct SearchHit<'a> {
    pub entry: &'a SearchEntry,
    pub score: i32,
}

/// Score tiers, see the module docs
const TIER_ID: i32 = 10_000;
const TIER_EXACT: i32 = 9_000;
const TIER_PREFIX: i32 = 8_000;
const TIER_WORD_PREFIX: i32 = 7_000;
const TIER_CONTAINS: i32 = 6_000;
const TIER_ALL_WORD_PREFIXES: i32 = 5_000;
const TIER_ALL_WORDS: i32 = 4_000;
const TIER_TYPO: i32 = 3_000;
const TIER_ID_PREFIX: i32 = 2_000;

/// Lowest score at which a typed name is trusted to mean that entry (up to two typos)
const RESOLVE_THRESHOLD: i32 = TIER_TYPO - 400;

/// Maximum edit distance accepted for a word of this length
fn typo_budget(len: usize) -> usize {
    match len {
        0..=3 => 0,
        4..=7 => 1,
        _ => 2,
    }
}

fn score(entry: &SearchEntry, query: &str, tokens: &[&str], numeric: Option<i32>) -> Option<i32> {
    if numeric == Some(entry.id) {
        return Some(TIER_ID);
    }
    let norm = entry.norm.as_str();
    let tier = if norm == query {
        TIER_EXACT
    } else if entry.aliases.iter().any(|a| a == query) {
        TIER_EXACT - 300
    } else if norm.starts_with(query) {
        TIER_PREFIX
    } else if norm.split(' ').any(|w| w.starts_with(query)) || norm.contains(&format!(" {query}")) {
        TIER_WORD_PREFIX
    } else if norm.contains(query) {
        TIER_CONTAINS
    } else if entry.aliases.iter().any(|a| a.starts_with(query) || a.contains(query)) {
        TIER_CONTAINS - 300
    } else if tokens.iter().all(|t| entry.words().any(|w| w.starts_with(t))) {
        TIER_ALL_WORD_PREFIXES
    } else if tokens.iter().all(|t| entry.texts().any(|text| text.contains(t))) {
        TIER_ALL_WORDS
    } else if query.len() >= 4 && entry.compact.contains(&query.replace(' ', "")) {
        TIER_ALL_WORDS - 100
    } else {
        // every token must be present, or be within a small edit distance of some word
        let mut distance = 0;
        for token in tokens {
            if entry.texts().any(|text| text.contains(token)) {
                continue;
            }
            let budget = typo_budget(token.chars().count());
            let best = entry
                .words()
                .filter(|w| !w.is_empty())
                .map(|w| {
                    // also compare against the start of longer words, so a typo while still
                    // typing (`"overlrd"` for `"overlords"`) is found
                    let len = token.chars().count();
                    let prefix_distance = (len.saturating_sub(1)..=len + 1)
                        .filter(|&n| n > 0 && n < w.chars().count())
                        .map(|n| {
                            let prefix: String = w.chars().take(n).collect();
                            strsim::damerau_levenshtein(token, &prefix)
                        })
                        .min()
                        .unwrap_or(usize::MAX);
                    strsim::damerau_levenshtein(token, w).min(prefix_distance)
                })
                .min()?;
            if best > budget {
                return match numeric {
                    Some(_) if entry.id.to_string().starts_with(query) => Some(TIER_ID_PREFIX),
                    _ => None,
                };
            }
            distance += best;
        }
        TIER_TYPO - 200 * distance as i32
    };
    Some(tier)
}

#[derive(Default)]
pub struct SearchIndex {
    categories: HashMap<SearchCategory, Vec<SearchEntry>>,
}

impl SearchIndex {
    pub fn insert(&mut self, category: SearchCategory, entries: Vec<SearchEntry>) {
        let mut entries = entries;
        entries.sort_by(|a, b| a.name.cmp(&b.name).then(a.id.cmp(&b.id)));
        self.categories.insert(category, entries);
    }

    pub fn len(&self, category: SearchCategory) -> usize {
        self.categories.get(&category).map_or(0, Vec::len)
    }

    pub fn get(&self, category: SearchCategory, id: i32) -> Option<&SearchEntry> {
        self.categories.get(&category)?.iter().find(|e| e.id == id)
    }

    /// The best matches for `query`, best first
    pub fn search(&self, category: SearchCategory, query: &str, limit: usize) -> Vec<SearchHit<'_>> {
        let Some(entries) = self.categories.get(&category) else {
            return vec![];
        };
        let query = normalize(query);
        if query.is_empty() {
            // nothing typed yet: suggest well known entries alphabetically
            let mut hits: Vec<_> = entries.iter().map(|entry| SearchHit { entry, score: entry.bonus }).collect();
            hits.sort_by_key(|hit| std::cmp::Reverse(hit.score));
            hits.truncate(limit);
            return hits;
        }
        let tokens: Vec<&str> = query.split(' ').collect();
        let numeric = query.parse::<i32>().ok();

        let mut hits: Vec<SearchHit> = entries
            .iter()
            .filter_map(|entry| {
                score(entry, &query, &tokens, numeric).map(|tier| SearchHit {
                    entry,
                    score: tier + entry.bonus,
                })
            })
            .collect();
        hits.sort_by(|a, b| {
            b.score
                .cmp(&a.score)
                .then(a.entry.name.len().cmp(&b.entry.name.len()))
                .then(a.entry.id.cmp(&b.entry.id))
        });
        hits.truncate(limit);
        hits
    }

    pub fn autocomplete(&self, category: SearchCategory, query: &str) -> Vec<AutocompleteChoice> {
        self.search(category, query, AUTOCOMPLETE_LIMIT)
            .into_iter()
            .map(|hit| AutocompleteChoice::new(hit.entry.label(), hit.entry.id.to_string()))
            .collect()
    }

    /// Turns what the user typed into an id
    /// - an autocomplete choice's value (`"1727"`) or a pasted label (`"Bat Lord Cape [1727]"`)
    /// - otherwise the best search match, when it is a confident match
    ///
    /// Fails with suggestions when nothing matches well
    pub fn resolve(&self, category: SearchCategory, input: &str) -> BotResult<i32> {
        let input = input.trim();
        if input.is_empty() {
            return Err(BotError::invalid(format!(
                "Please enter the name or id of a{} {}.",
                if category.label().starts_with(['a', 'e', 'i', 'o', 'u']) { "n" } else { "" },
                category.label()
            )));
        }
        if let Ok(id) = input.trim_start_matches('#').parse::<i32>() {
            return Ok(id);
        }
        if let Some(id) = trailing_bracket_id(input) {
            return Ok(id);
        }

        let hits = self.search(category, input, 5);
        match hits.first() {
            Some(best) if best.score >= RESOLVE_THRESHOLD => Ok(best.entry.id),
            Some(_) => Err(not_found_with_suggestions(category, input, &hits)),
            None => Err(not_found_with_suggestions(category, input, &self.suggest(category, input, 5))),
        }
    }

    /// Loose matches for "did you mean": entries sharing the most words with the query
    pub fn suggest(&self, category: SearchCategory, query: &str, limit: usize) -> Vec<SearchHit<'_>> {
        let Some(entries) = self.categories.get(&category) else {
            return vec![];
        };
        let query = normalize(query);
        let tokens: Vec<&str> = query.split(' ').filter(|t| t.chars().count() >= 2).collect();
        if tokens.is_empty() {
            return vec![];
        }
        let mut hits: Vec<SearchHit> = entries
            .iter()
            .filter_map(|entry| {
                let matched = tokens
                    .iter()
                    .filter(|t| entry.words().any(|w| w.starts_with(*t)))
                    .count() as i32;
                (matched > 0).then(|| SearchHit {
                    entry,
                    score: matched * 100 + entry.bonus / 10,
                })
            })
            .collect();
        hits.sort_by(|a, b| {
            b.score
                .cmp(&a.score)
                .then(a.entry.name.len().cmp(&b.entry.name.len()))
                .then(a.entry.id.cmp(&b.entry.id))
        });
        hits.truncate(limit);
        hits
    }
}

/// Parses the id out of `"Some Name [1234]"`
fn trailing_bracket_id(input: &str) -> Option<i32> {
    let inner = input.strip_suffix(']')?;
    let open = inner.rfind('[')?;
    inner[open + 1..].trim().trim_matches('`').parse().ok()
}

fn not_found_with_suggestions(category: SearchCategory, input: &str, hits: &[SearchHit]) -> BotError {
    let mut msg = format!("No {} matches `{}`.", category.label(), sanitize_inline(input));
    if !hits.is_empty() {
        msg.push_str("\nDid you mean:");
        for hit in hits {
            msg.push_str(&format!("\n- {}", hit.entry.label()));
        }
    } else {
        msg.push_str("\nTry picking one of the autocomplete suggestions, or use `/search`.");
    }
    BotError::not_found(msg)
}

/// Makes user text safe to put inside inline code
pub fn sanitize_inline(text: impl Display) -> String {
    truncate(&text.to_string().replace('`', "'"), 80)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn index() -> SearchIndex {
        let mut index = SearchIndex::default();
        index.insert(
            SearchCategory::Item,
            vec![
                SearchEntry::new(36052, "Overlord's Helmet").bonus(100),
                SearchEntry::new(37000, "Bat Lord Cape").alias("Halloween Cape 2").bonus(100),
                SearchEntry::new(7415, "Bat Lord Helm").bonus(100),
                SearchEntry::new(1727, "Shark Mask"),
                SearchEntry::new(12345, "Dark Ronin Sword"),
                SearchEntry::new(123, "Cape"),
            ],
        );
        index
    }

    fn top(query: &str) -> Option<i32> {
        index().search(SearchCategory::Item, query, 5).first().map(|h| h.entry.id)
    }

    #[test]
    fn normalizes() {
        assert_eq!(normalize("Overlord's  Helmet!"), "overlords helmet");
        assert_eq!(normalize("  Bat-Lord_Cape "), "bat lord cape");
    }

    #[test]
    fn ranking() {
        assert_eq!(top("cape"), Some(123)); // exact beats prefix
        assert_eq!(top("bat lord c"), Some(37000));
        assert_eq!(top("BAT"), Some(7415)); // case insensitive; equal length ties go to the lower id
        assert_eq!(top("overlords"), Some(36052)); // apostrophe ignored
        assert_eq!(top("helmet overlord"), Some(36052)); // word order
        assert_eq!(top("ronin swrd"), Some(12345)); // typo
        assert_eq!(top("overlrd"), Some(36052)); // typo while typing
        assert_eq!(top("halloween"), Some(37000)); // alias
        assert_eq!(top("1727"), Some(1727)); // id
        assert_eq!(top("batlord"), Some(7415)); // missing space
        assert_eq!(top("bat lrod cape"), Some(37000)); // transposed letters
        assert_eq!(top("zzzzzz"), None);
    }

    #[test]
    fn resolving() {
        let index = index();
        assert_eq!(index.resolve(SearchCategory::Item, "37000").unwrap(), 37000);
        assert_eq!(index.resolve(SearchCategory::Item, "Bat Lord Cape [37000]").unwrap(), 37000);
        assert_eq!(index.resolve(SearchCategory::Item, "shark mask").unwrap(), 1727);
        assert_eq!(index.resolve(SearchCategory::Item, "shrak mask").unwrap(), 1727);
        let err = index.resolve(SearchCategory::Item, "bat lord sword").unwrap_err();
        assert!(err.to_string().contains("Bat Lord Cape [37000]"), "{err}");
        let err = index.resolve(SearchCategory::Item, "qqqq").unwrap_err();
        assert!(matches!(err, BotError::NotFound(_)));
        assert!(index.resolve(SearchCategory::Item, "  ").is_err());
    }

    #[test]
    fn labels_fit() {
        let entry = SearchEntry::new(1, "x".repeat(300));
        assert!(entry.label().chars().count() <= 100);
        assert!(entry.label().ends_with(" [1]"));
    }
}
