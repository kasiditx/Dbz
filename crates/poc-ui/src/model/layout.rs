/// Resizable split (sidebar|main or editor|results) stored as a ratio so window
/// resizes keep proportions.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Split {
    pub ratio: f32,
}

impl Split {
    pub fn new(ratio: f32) -> Self {
        Self { ratio }
    }
    /// Pixel position of the divider in `total`, respecting min sizes of both panes.
    pub fn pos(&self, total: f32, min_a: f32, min_b: f32) -> f32 {
        let total = total.max(0.0);
        if total < min_a + min_b {
            return total * min_a / (min_a + min_b);
        }
        (total * self.ratio).clamp(min_a, total - min_b)
    }
    /// Drag divider to `pos`; stores clamped ratio.
    pub fn drag_to(&mut self, pos: f32, total: f32, min_a: f32, min_b: f32) {
        if total <= 0.0 || total < min_a + min_b {
            return;
        }
        self.ratio = pos.clamp(min_a, total - min_b) / total;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pos_from_ratio() {
        assert_eq!(Split::new(0.25).pos(1000.0, 100.0, 100.0), 250.0);
    }

    #[test]
    fn pos_clamps_to_minimums() {
        assert_eq!(Split::new(0.01).pos(1000.0, 100.0, 100.0), 100.0);
        assert_eq!(Split::new(0.99).pos(1000.0, 100.0, 100.0), 900.0);
    }

    #[test]
    fn ratio_survives_window_resize() {
        let mut s = Split::new(0.5);
        s.drag_to(300.0, 1000.0, 100.0, 100.0);
        assert_eq!(s.pos(1000.0, 100.0, 100.0), 300.0);
        assert_eq!(s.pos(500.0, 100.0, 100.0), 150.0);
    }

    #[test]
    fn drag_beyond_bounds_is_clamped() {
        let mut s = Split::new(0.5);
        s.drag_to(-50.0, 1000.0, 100.0, 100.0);
        assert_eq!(s.pos(1000.0, 100.0, 100.0), 100.0);
        s.drag_to(5000.0, 1000.0, 100.0, 100.0);
        assert_eq!(s.pos(1000.0, 100.0, 100.0), 900.0);
    }

    #[test]
    fn tiny_window_smaller_than_minimums_stays_finite_and_proportional() {
        let p = Split::new(0.5).pos(100.0, 100.0, 100.0);
        assert!(p.is_finite());
        assert_eq!(p, 50.0);
        assert_eq!(Split::new(0.5).pos(0.0, 100.0, 100.0), 0.0);
    }
}
