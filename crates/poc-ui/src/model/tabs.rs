#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tab {
    pub id: u64,
    pub title: String,
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct Tabs {
    pub tabs: Vec<Tab>,
    pub active: usize,
    next_id: u64,
}

impl Tabs {
    /// Starts with one tab so the editor is never empty.
    pub fn new() -> Self {
        let mut t = Tabs {
            tabs: Vec::new(),
            active: 0,
            next_id: 1,
        };
        t.open("");
        t
    }
    pub fn open(&mut self, text: &str) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.tabs.push(Tab {
            id,
            title: format!("SQL {id}"),
            text: text.into(),
        });
        self.active = self.tabs.len() - 1;
        self.active
    }
    /// Close tab `idx`; the last remaining tab cannot be closed. Returns closed?
    pub fn close(&mut self, idx: usize) -> bool {
        if self.tabs.len() <= 1 || idx >= self.tabs.len() {
            return false;
        }
        self.tabs.remove(idx);
        if idx < self.active {
            self.active -= 1;
        }
        self.active = self.active.min(self.tabs.len() - 1);
        true
    }
    pub fn next(&mut self) {
        self.active = (self.active + 1) % self.tabs.len();
    }
    pub fn prev(&mut self) {
        self.active = (self.active + self.tabs.len() - 1) % self.tabs.len();
    }
    pub fn active_mut(&mut self) -> &mut Tab {
        &mut self.tabs[self.active]
    }
}

impl Default for Tabs {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_with_one_tab() {
        let t = Tabs::new();
        assert_eq!(t.tabs.len(), 1);
        assert_eq!(t.tabs[0].title, "SQL 1");
        assert_eq!(t.active, 0);
    }

    #[test]
    fn open_adds_and_activates_with_unique_ids_and_titles() {
        let mut t = Tabs::new();
        let i = t.open("select 1 from dual");
        assert_eq!(i, 1);
        assert_eq!(t.active, 1);
        assert_eq!(t.tabs[1].title, "SQL 2");
        assert_eq!(t.tabs[1].text, "select 1 from dual");
        assert_ne!(t.tabs[0].id, t.tabs[1].id);
    }

    #[test]
    fn close_last_remaining_is_refused() {
        let mut t = Tabs::new();
        assert!(!t.close(0));
        assert_eq!(t.tabs.len(), 1);
    }

    #[test]
    fn close_before_active_shifts_active_index() {
        let mut t = Tabs::new();
        t.open("");
        t.open("");
        assert_eq!(t.active, 2);
        assert!(t.close(0));
        assert_eq!(t.active, 1);
        assert_eq!(t.tabs.len(), 2);
    }

    #[test]
    fn close_active_last_selects_new_last() {
        let mut t = Tabs::new();
        t.open("");
        assert!(t.close(1));
        assert_eq!(t.active, 0);
    }

    #[test]
    fn close_out_of_range_is_noop() {
        let mut t = Tabs::new();
        t.open("");
        assert!(!t.close(9));
        assert_eq!(t.tabs.len(), 2);
    }

    #[test]
    fn next_prev_wrap() {
        let mut t = Tabs::new();
        t.open("");
        t.open("");
        t.next();
        assert_eq!(t.active, 0);
        t.prev();
        assert_eq!(t.active, 2);
    }

    #[test]
    fn titles_not_reused_after_close() {
        let mut t = Tabs::new();
        t.open("");
        t.close(1);
        t.open("");
        assert_eq!(t.tabs[1].title, "SQL 3");
    }
}
