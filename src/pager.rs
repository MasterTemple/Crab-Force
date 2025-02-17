/*
* - This I believe should cover pages where it is just Discord Embed Fields
* - This doesn't paginate the Discord Embed Description
*/

/// Page starts at 1
pub const MIN_PAGE: usize = 1;
/// This is an alias for [`MIN_PAGE`]
pub const START_PAGE: usize = MIN_PAGE;

/// This is discords field limit
// pub const PAGE_SIZE: usize = 15;

/// Page starts at 1
#[derive(Clone, Debug)]
pub struct Pager<T: Clone> {
    entries: Vec<T>,
    current: usize,
    min_page: usize,
    max_page: usize,
    page_size: usize,
}

impl<T: Clone> Pager<T> {
    pub fn new(entries: Vec<T>, page: usize, page_size: usize) -> Self {
        Self {
            current: page,
            min_page: MIN_PAGE,
            max_page: (entries.len() / page_size)
                + (if entries.len() % page_size == 0 { 0 } else { 1 }),
            page_size,
            entries,
        }
    }

    pub fn prev(&self) -> usize {
        std::cmp::max(self.current - 1, self.min_page)
    }

    pub fn is_first_page(&self) -> bool {
        self.current == self.min_page
    }

    pub fn next(&self) -> usize {
        std::cmp::min(self.current + 1, self.max_page)
    }

    pub fn is_last_page(&self) -> bool {
        self.current == self.max_page
    }

    pub fn has_multiple_pages(&self) -> bool {
        self.min_page != self.max_page
    }

    pub fn is_one_page(&self) -> bool {
        self.min_page == self.max_page
    }

    /// This returns all elements on this page with their 1-based entry number and entry
    pub fn this_page(&self) -> Vec<(usize, T)> {
        // page starts at 1
        let start_idx = (self.current - 1) * self.page_size;
        let end_idx = std::cmp::min(start_idx + self.page_size, self.entries.len());
        let paged_entries: Vec<_> = self.entries[start_idx..end_idx]
            .into_iter()
            .enumerate()
            .map(|(idx, entry)| (start_idx + idx + 1, entry.clone()))
            .collect();
        paged_entries
    }
}
