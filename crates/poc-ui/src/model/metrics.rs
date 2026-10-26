/// Bounded sample window with nearest-rank percentiles (ms).
#[derive(Debug, Clone)]
pub struct Samples {
    cap: usize,
    data: Vec<f32>,
}

impl Samples {
    pub fn new(cap: usize) -> Self {
        Self {
            cap,
            data: Vec::new(),
        }
    }
    pub fn push(&mut self, v: f32) {
        if self.cap > 0 && self.data.len() == self.cap {
            self.data.remove(0);
        }
        self.data.push(v);
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    /// Nearest-rank percentile, p in 0..=100.
    pub fn percentile(&self, p: f32) -> Option<f32> {
        if self.data.is_empty() {
            return None;
        }
        let mut v = self.data.clone();
        v.sort_by(|a, b| a.total_cmp(b));
        let rank = ((p.clamp(0.0, 100.0) / 100.0) * v.len() as f32).ceil() as usize;
        Some(v[rank.max(1) - 1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn filled(n: usize) -> Samples {
        let mut s = Samples::new(1000);
        for i in 1..=n {
            s.push(i as f32);
        }
        s
    }

    #[test]
    fn nearest_rank_percentiles() {
        let s = filled(100);
        assert_eq!(s.percentile(50.0), Some(50.0));
        assert_eq!(s.percentile(95.0), Some(95.0));
        assert_eq!(s.percentile(100.0), Some(100.0));
        assert_eq!(s.percentile(0.0), Some(1.0));
    }

    #[test]
    fn unsorted_input() {
        let mut s = Samples::new(10);
        for v in [9.0, 1.0, 5.0] {
            s.push(v);
        }
        assert_eq!(s.percentile(50.0), Some(5.0));
    }

    #[test]
    fn empty_is_none() {
        assert_eq!(Samples::new(5).percentile(50.0), None);
    }

    #[test]
    fn cap_drops_oldest() {
        let mut s = Samples::new(3);
        for v in 1..=5 {
            s.push(v as f32);
        }
        assert_eq!(s.len(), 3);
        assert_eq!(s.percentile(0.0), Some(3.0));
    }
}
