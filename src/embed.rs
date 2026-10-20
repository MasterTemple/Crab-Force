//! An embed builder that always produces embeds Discord will accept
//!
//! Discord rejects the *whole* message when any embed limit is exceeded or a field is empty, so
//! every string is truncated here and the 6000 character total is budgeted when building.

use serenity::all::{Colour, CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter};

use crate::config::config;

pub const TITLE_LIMIT: usize = 256;
pub const DESCRIPTION_LIMIT: usize = 4096;
pub const FIELD_NAME_LIMIT: usize = 256;
pub const FIELD_VALUE_LIMIT: usize = 1024;
pub const FIELD_COUNT_LIMIT: usize = 25;
pub const TOTAL_LIMIT: usize = 6000;
pub const FOOTER_LIMIT: usize = 2048;
pub const AUTHOR_LIMIT: usize = 256;

/// A zero width space, for when Discord requires text but there is nothing to say
pub const BLANK: &str = "\u{200b}";

/// Truncates to at most `max` characters (not bytes), adding `…` when anything was cut
pub fn truncate(text: &str, max: usize) -> String {
    if text.chars().count() <= max {
        return text.to_string();
    }
    let mut out: String = text.chars().take(max.saturating_sub(1)).collect();
    out.push('…');
    out
}

/// Like [`truncate`] but prefers cutting at a line break so list entries are not split
pub fn truncate_lines(text: &str, max: usize) -> String {
    if text.chars().count() <= max {
        return text.to_string();
    }
    let suffix = "\n…";
    let budget = max.saturating_sub(suffix.chars().count());
    let mut out = String::new();
    let mut count = 0;
    for line in text.split('\n') {
        let len = line.chars().count() + usize::from(!out.is_empty());
        if count + len > budget {
            break;
        }
        if !out.is_empty() {
            out.push('\n');
        }
        out.push_str(line);
        count += len;
    }
    if out.is_empty() {
        // the first line alone is too long
        return truncate(text, max);
    }
    out.push_str(suffix);
    out
}

fn non_empty(text: String) -> String {
    if text.trim().is_empty() {
        String::from(BLANK)
    } else {
        text
    }
}

#[derive(Clone, Debug)]
pub struct Field {
    pub name: String,
    pub value: String,
    pub inline: bool,
}

#[derive(Clone, Debug, Default)]
pub struct Embed {
    title: Option<String>,
    url: Option<String>,
    description: Option<String>,
    thumbnail: Option<String>,
    color: Option<Colour>,
    fields: Vec<Field>,
    /// Fields that did not fit (only counted, for the "and N more" note)
    overflow: usize,
}

impl Embed {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(truncate(&title.into(), TITLE_LIMIT));
        self
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        let description = description.into();
        if !description.trim().is_empty() {
            self.description = Some(truncate_lines(&description, DESCRIPTION_LIMIT));
        }
        self
    }

    pub fn thumbnail(mut self, url: Option<String>) -> Self {
        self.thumbnail = url;
        self
    }

    pub fn color(mut self, color: Colour) -> Self {
        self.color = Some(color);
        self
    }

    pub fn field(mut self, name: impl Into<String>, value: impl Into<String>, inline: bool) -> Self {
        self.push_field(name, value, inline);
        self
    }

    pub fn push_field(&mut self, name: impl Into<String>, value: impl Into<String>, inline: bool) {
        if self.fields.len() >= FIELD_COUNT_LIMIT {
            self.overflow += 1;
            return;
        }
        self.fields.push(Field {
            name: non_empty(truncate(&name.into(), FIELD_NAME_LIMIT)),
            value: non_empty(truncate_lines(&value.into(), FIELD_VALUE_LIMIT)),
            inline,
        });
    }

    /// Adds `lines` as one or more fields, splitting at line boundaries so nothing is cut off
    /// - Continuation fields get a blank name so they read as one list
    pub fn field_lines(mut self, name: impl Into<String>, lines: &[String], inline: bool) -> Self {
        let name = name.into();
        if lines.is_empty() {
            return self.field(name, "None", inline);
        }
        let mut chunk = String::new();
        let mut first = true;
        for line in lines {
            let line = truncate(line, FIELD_VALUE_LIMIT);
            if !chunk.is_empty() && chunk.chars().count() + 1 + line.chars().count() > FIELD_VALUE_LIMIT {
                let field_name = if first { name.clone() } else { String::from(BLANK) };
                self.push_field(field_name, std::mem::take(&mut chunk), inline);
                first = false;
            }
            if !chunk.is_empty() {
                chunk.push('\n');
            }
            chunk.push_str(&line);
        }
        let field_name = if first { name } else { String::from(BLANK) };
        self.push_field(field_name, chunk, inline);
        self
    }

    pub fn field_count(&self) -> usize {
        self.fields.len()
    }

    /// Converts to serenity's embed, adding the configured author/footer and making sure the
    /// total length stays under Discord's limit (trailing fields are dropped if needed)
    pub fn build(self) -> CreateEmbed {
        let config = config();
        let mut embed = CreateEmbed::new().color(self.color.unwrap_or(config.color));
        let mut total = 0;

        if let Some(name) = &config.author_name {
            let name = truncate(name, AUTHOR_LIMIT);
            total += name.chars().count();
            let mut author = CreateEmbedAuthor::new(name);
            if let Some(icon) = &config.author_icon {
                author = author.icon_url(icon);
            }
            if let Some(url) = &config.author_url {
                author = author.url(url);
            }
            embed = embed.author(author);
        }

        if let Some(text) = &config.footer_text {
            let text = truncate(text, FOOTER_LIMIT);
            total += text.chars().count();
            let mut footer = CreateEmbedFooter::new(text);
            if let Some(icon) = &config.footer_icon {
                footer = footer.icon_url(icon);
            }
            embed = embed.footer(footer);
        }

        if let Some(title) = self.title {
            total += title.chars().count();
            embed = embed.title(title);
        }
        if let Some(url) = self.url {
            embed = embed.url(url);
        }
        if let Some(thumbnail) = self.thumbnail {
            embed = embed.thumbnail(thumbnail);
        }

        // reserve room for a possible "and N more" note
        let note_reserve = 64;
        if let Some(description) = self.description {
            let room = TOTAL_LIMIT.saturating_sub(total + note_reserve);
            let description = truncate_lines(&description, room.min(DESCRIPTION_LIMIT));
            total += description.chars().count();
            embed = embed.description(description);
        }

        let mut dropped = self.overflow;
        let mut fields = vec![];
        for field in self.fields {
            let len = field.name.chars().count() + field.value.chars().count();
            if dropped > 0 || total + len + note_reserve > TOTAL_LIMIT {
                dropped += 1;
                continue;
            }
            total += len;
            fields.push((field.name, field.value, field.inline));
        }
        if dropped > 0 && fields.len() < FIELD_COUNT_LIMIT {
            fields.push((
                String::from(BLANK),
                format!("*…and {dropped} more not shown (Discord's message size limit)*"),
                false,
            ));
        }
        embed.fields(fields)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncates_by_chars() {
        assert_eq!(truncate("héllo", 10), "héllo");
        assert_eq!(truncate("héllo", 3), "hé…");
    }

    #[test]
    fn truncates_at_lines() {
        let text = "aaaa\nbbbb\ncccc";
        assert_eq!(truncate_lines(text, 11), "aaaa\nbbbb\n…");
        assert_eq!(truncate_lines(text, 10), "aaaa\n…");
        assert_eq!(truncate_lines(text, 100), text);
    }

    #[test]
    fn splits_long_lists() {
        let lines: Vec<String> = (0..200).map(|i| format!("**{i}.** Some Item Name [`{i}`]")).collect();
        let embed = Embed::new().field_lines("Items", &lines, false);
        assert!(embed.field_count() > 1);
        assert!(embed.fields.iter().all(|f| f.value.chars().count() <= FIELD_VALUE_LIMIT));
        assert!(embed.fields.iter().all(|f| !f.value.is_empty() && !f.name.is_empty()));
    }

    #[test]
    fn empty_values_are_replaced() {
        let embed = Embed::new().field("", "", true);
        assert_eq!(embed.fields[0].name, BLANK);
        assert_eq!(embed.fields[0].value, BLANK);
    }
}
