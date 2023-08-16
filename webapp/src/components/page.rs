#[derive(Clone, PartialEq)]
pub struct PageOrganizer<const PER_VIEW: usize> {
    pub current_bounds: [(usize, usize); PER_VIEW],
    pub current_page: usize,
    pub last_page: usize,
}

impl<const PER_VIEW: usize> PageOrganizer<PER_VIEW> {
    pub fn new(per_page: usize, page: usize, total: usize) -> Self {
        let mut open_pages = [(0, 0); PER_VIEW];
        let mut start = per_page * page;
        let last_page = (total + per_page * PER_VIEW - 1) / (per_page * PER_VIEW);
        for (st, end) in &mut open_pages {
            *st = start;
            start = (start + per_page).min(total);
            *end = start - 1;
        }
        Self {
            current_bounds: open_pages,
            current_page: page,
            last_page,
        }
    }

    pub fn page_display_id(&self) -> usize {
        1 + self.current_page / PER_VIEW
    }
}
