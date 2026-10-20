//! Splits a list of entries into pages (pages start at 1)
//!
//! The requested page is clamped into range, so stale buttons (e.g. after `/reload` shrinks a
//! list) show the nearest page instead of panicking.

/// Page starts at 1
pub const MIN_PAGE: usize = 1;
/// This is an alias for [`MIN_PAGE`]
pub const START_PAGE: usize = MIN_PAGE;

#[derive(Clone, Debug)]
pub struct Pager<T> {
    entries: Vec<T>,
    current: usize,
    max_page: usize,
    page_size: usize,
}

impl<T> Pager<T> {
    pub fn new(entries: Vec<T>, page: usize, page_size: usize) -> Self {
        let page_size = page_size.max(1);
        let max_page = entries.len().div_ceil(page_size).max(MIN_PAGE);
        Self {
            current: page.clamp(MIN_PAGE, max_page),
            max_page,
            page_size,
            entries,
        }
    }

    pub fn current(&self) -> usize {
        self.current
    }

    pub fn max_page(&self) -> usize {
        self.max_page
    }

    pub fn total(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn prev(&self) -> usize {
        self.current.saturating_sub(1).max(MIN_PAGE)
    }

    pub fn next(&self) -> usize {
        (self.current + 1).min(self.max_page)
    }

    pub fn is_first_page(&self) -> bool {
        self.current == MIN_PAGE
    }

    pub fn is_last_page(&self) -> bool {
        self.current == self.max_page
    }

    pub fn has_multiple_pages(&self) -> bool {
        self.max_page > MIN_PAGE
    }

    /// `" (Page 2/5)"`, or nothing when there is only one page
    pub fn title_suffix(&self) -> String {
        if self.has_multiple_pages() {
            format!(" (Page {}/{})", self.current, self.max_page)
        } else {
            String::new()
        }
    }

    /// All elements on this page with their 1-based entry number
    pub fn this_page(&self) -> impl Iterator<Item = (usize, &T)> {
        let start = (self.current - 1) * self.page_size;
        let end = (start + self.page_size).min(self.entries.len());
        self.entries[start.min(end)..end]
            .iter()
            .enumerate()
            .map(move |(idx, entry)| (start + idx + 1, entry))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamps_pages() {
        let pager = Pager::new((1..=11).collect::<Vec<_>>(), 99, 5);
        assert_eq!(pager.current(), 3);
        assert_eq!(pager.this_page().map(|(n, &v)| (n, v)).collect::<Vec<_>>(), vec![(11, 11)]);

        let pager = Pager::new((1..=11).collect::<Vec<_>>(), 0, 5);
        assert_eq!(pager.current(), 1);
        assert_eq!(pager.prev(), 1);
        assert_eq!(pager.next(), 2);
    }

    #[test]
    fn empty() {
        let pager: Pager<i32> = Pager::new(vec![], 3, 5);
        assert_eq!(pager.current(), 1);
        assert!(!pager.has_multiple_pages());
        assert_eq!(pager.this_page().count(), 0);
        assert_eq!(pager.title_suffix(), "");
    }
}
