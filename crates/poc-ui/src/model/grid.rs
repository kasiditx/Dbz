use std::ops::Range;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cell {
    Null,
    Text(String),
    /// Exact decimal text (never f64).
    Number(String),
}

/// Row provider seam. Real DB pages or synthetic data sit behind this.
pub trait RowSource {
    fn row_count(&self) -> usize;
    fn col_count(&self) -> usize;
    fn column_name(&self, col: usize) -> String;
    /// Must be O(1)-ish and allocate only the requested cell.
    fn cell(&self, row: usize, col: usize) -> Cell;
}

/// Deterministic generated rows; stores no row data.
#[derive(Debug, Clone)]
pub struct SyntheticSource {
    rows: usize,
    cols: usize,
}

impl SyntheticSource {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self { rows, cols }
    }
}

impl RowSource for SyntheticSource {
    fn row_count(&self) -> usize {
        self.rows
    }
    fn col_count(&self) -> usize {
        self.cols
    }
    fn column_name(&self, col: usize) -> String {
        match col {
            0 => "ID".into(),
            1 => "CUSTOMER_NAME".into(),
            2 => "AMOUNT".into(),
            3 => "STATUS".into(),
            4 => "NOTES".into(),
            n => format!("COL_{n}"),
        }
    }
    fn cell(&self, row: usize, col: usize) -> Cell {
        match col {
            0 => Cell::Number(row.to_string()),
            1 => Cell::Text(format!("ลูกค้า {row}")),
            2 => Cell::Number(format!("{}.{:09}", row * 7919, row % 1_000_000_000)),
            3 if row.is_multiple_of(7) => Cell::Null,
            3 => Cell::Text(["OPEN", "PAID", "VOID"][row % 3].into()),
            4 => Cell::Text(format!("หมายเหตุยาว {row} ").repeat(30)),
            n => Cell::Text(format!("r{row}c{n}")),
        }
    }
}

/// Rows to materialize for a viewport (with overscan), clamped to `total`.
pub fn visible_range(
    scroll_y: f32,
    viewport_h: f32,
    row_h: f32,
    total: usize,
    overscan: usize,
) -> Range<usize> {
    if total == 0 || row_h <= 0.0 {
        return 0..0;
    }
    let top = scroll_y.max(0.0);
    let first = ((top / row_h).floor() as usize).min(total);
    let last = (((top + viewport_h.max(0.0)) / row_h).ceil() as usize).min(total);
    first.saturating_sub(overscan)..(last + overscan).min(total)
}

/// Fetch only the cells of `range`; the single path the UI uses to read cells.
pub fn visible_cells(src: &dyn RowSource, range: Range<usize>) -> Vec<Vec<Cell>> {
    let cols = src.col_count();
    range
        .map(|r| (0..cols).map(|c| src.cell(r, c)).collect())
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Nav {
    Up,
    Down,
    Left,
    Right,
    PageUp,
    PageDown,
    Home,
    End,
    CtrlHome,
    CtrlEnd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Selection {
    pub row: usize,
    pub col: usize,
}

impl Selection {
    pub fn apply(&mut self, nav: Nav, rows: usize, cols: usize, page: usize) {
        if rows == 0 || cols == 0 {
            return;
        }
        let (last_r, last_c) = (rows - 1, cols - 1);
        match nav {
            Nav::Up => self.row = self.row.saturating_sub(1),
            Nav::Down => self.row = (self.row + 1).min(last_r),
            Nav::Left => self.col = self.col.saturating_sub(1),
            Nav::Right => self.col = (self.col + 1).min(last_c),
            Nav::PageUp => self.row = self.row.saturating_sub(page),
            Nav::PageDown => self.row = (self.row + page).min(last_r),
            Nav::Home => self.col = 0,
            Nav::End => self.col = last_c,
            Nav::CtrlHome => *self = Selection { row: 0, col: 0 },
            Nav::CtrlEnd => {
                *self = Selection {
                    row: last_r,
                    col: last_c,
                }
            }
        }
    }
}

/// New scroll offset that keeps `row` fully visible; unchanged if already visible.
pub fn scroll_to_reveal(row: usize, scroll_y: f32, viewport_h: f32, row_h: f32) -> f32 {
    let top = row as f32 * row_h;
    let bottom = top + row_h;
    if top < scroll_y {
        top
    } else if bottom > scroll_y + viewport_h {
        bottom - viewport_h
    } else {
        scroll_y
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell as Counter;

    #[test]
    fn synthetic_cells_are_deterministic_and_typed() {
        let s = SyntheticSource::new(100, 20);
        assert_eq!(s.cell(0, 0), Cell::Number("0".into()));
        assert_eq!(s.cell(5, 1), Cell::Text("ลูกค้า 5".into()));
        assert_eq!(s.cell(7, 3), Cell::Null);
        assert!(matches!(s.cell(8, 3), Cell::Text(_)));
        match s.cell(3, 4) {
            Cell::Text(t) => assert!(t.chars().count() >= 300),
            other => panic!("{other:?}"),
        }
        assert_eq!(s.column_name(0), "ID");
    }

    #[test]
    fn decimal_column_keeps_exact_digits() {
        let s = SyntheticSource::new(10, 20);
        assert_eq!(s.cell(2, 2), Cell::Number("15838.000000002".into()));
    }

    #[test]
    fn million_row_source_is_constant_size_and_reachable() {
        let s = SyntheticSource::new(1_000_000, 20);
        assert_eq!(std::mem::size_of_val(&s), 2 * std::mem::size_of::<usize>());
        assert_eq!(s.row_count(), 1_000_000);
        assert_eq!(s.cell(999_999, 0), Cell::Number("999999".into()));
    }

    #[test]
    fn visible_range_basic_and_scrolled() {
        assert_eq!(visible_range(0.0, 100.0, 20.0, 1000, 0), 0..5);
        assert_eq!(visible_range(25.0, 100.0, 20.0, 1000, 0), 1..7);
    }

    #[test]
    fn visible_range_overscan_and_clamps() {
        assert_eq!(visible_range(100.0, 100.0, 20.0, 1000, 2), 3..12);
        assert_eq!(visible_range(0.0, 100.0, 20.0, 3, 0), 0..3);
        assert_eq!(visible_range(0.0, 100.0, 20.0, 0, 4), 0..0);
        assert_eq!(visible_range(-50.0, 100.0, 20.0, 1000, 0), 0..5);
        assert_eq!(visible_range(1_000_000.0, 100.0, 20.0, 10, 0).end, 10);
    }

    #[test]
    fn visible_range_at_1m_rows_bottom_is_small() {
        let total = 1_000_000;
        let r = visible_range((total as f32 - 40.0) * 22.0, 880.0, 22.0, total, 3);
        assert!(r.len() <= 40 + 6 + 1, "len {}", r.len());
        assert_eq!(r.end, total);
    }

    struct Counting {
        inner: SyntheticSource,
        calls: Counter<usize>,
    }
    impl RowSource for Counting {
        fn row_count(&self) -> usize {
            self.inner.row_count()
        }
        fn col_count(&self) -> usize {
            self.inner.col_count()
        }
        fn column_name(&self, c: usize) -> String {
            self.inner.column_name(c)
        }
        fn cell(&self, r: usize, c: usize) -> Cell {
            self.calls.set(self.calls.get() + 1);
            self.inner.cell(r, c)
        }
    }

    #[test]
    fn only_visible_cells_are_read_from_source() {
        let src = Counting {
            inner: SyntheticSource::new(1_000_000, 20),
            calls: Counter::new(0),
        };
        let range = visible_range(500_000.0, 880.0, 22.0, src.row_count(), 2);
        let cells = visible_cells(&src, range.clone());
        assert_eq!(cells.len(), range.len());
        assert_eq!(src.calls.get(), range.len() * 20);
        assert!(src.calls.get() < 1_000);
    }

    #[test]
    fn selection_moves_and_clamps() {
        let mut s = Selection::default();
        s.apply(Nav::Up, 10, 5, 3);
        s.apply(Nav::Left, 10, 5, 3);
        assert_eq!(s, Selection { row: 0, col: 0 });
        s.apply(Nav::Down, 10, 5, 3);
        s.apply(Nav::Right, 10, 5, 3);
        assert_eq!(s, Selection { row: 1, col: 1 });
        for _ in 0..20 {
            s.apply(Nav::Down, 10, 5, 3);
            s.apply(Nav::Right, 10, 5, 3);
        }
        assert_eq!(s, Selection { row: 9, col: 4 });
    }

    #[test]
    fn selection_page_home_end() {
        let mut s = Selection { row: 5, col: 2 };
        s.apply(Nav::PageDown, 100, 5, 10);
        assert_eq!(s.row, 15);
        s.apply(Nav::PageUp, 100, 5, 10);
        s.apply(Nav::PageUp, 100, 5, 10);
        assert_eq!(s.row, 0);
        s.apply(Nav::End, 100, 5, 10);
        assert_eq!(s.col, 4);
        s.apply(Nav::Home, 100, 5, 10);
        assert_eq!(s.col, 0);
        s.apply(Nav::CtrlEnd, 100, 5, 10);
        assert_eq!(s, Selection { row: 99, col: 4 });
        s.apply(Nav::CtrlHome, 100, 5, 10);
        assert_eq!(s, Selection { row: 0, col: 0 });
    }

    #[test]
    fn selection_on_empty_grid_is_noop() {
        let mut s = Selection::default();
        for n in [Nav::Down, Nav::Right, Nav::PageDown, Nav::CtrlEnd, Nav::End] {
            s.apply(n, 0, 0, 10);
        }
        assert_eq!(s, Selection { row: 0, col: 0 });
    }

    #[test]
    fn scroll_to_reveal_cases() {
        // viewport 100, rows 20px
        assert_eq!(scroll_to_reveal(2, 0.0, 100.0, 20.0), 0.0); // visible
        assert_eq!(scroll_to_reveal(10, 0.0, 100.0, 20.0), 120.0); // below: bottom edge at 220-100
        assert_eq!(scroll_to_reveal(1, 100.0, 100.0, 20.0), 20.0); // above
    }
}
