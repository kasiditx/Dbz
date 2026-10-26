//! Scripted benchmark driver. Emits raw per-frame CSV; no numbers are computed here beyond
//! what is directly timed. Percentiles are computed by benchmarks/summarize.py from raw rows.
use std::fmt::Write as _;
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scenario {
    Startup,
    Scroll,
    Nav,
    Resize,
    Idle,
}

impl Scenario {
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "startup" => Some(Self::Startup),
            "scroll" => Some(Self::Scroll),
            "nav" => Some(Self::Nav),
            "resize" => Some(Self::Resize),
            "idle" => Some(Self::Idle),
            _ => None,
        }
    }
    pub fn name(self) -> &'static str {
        match self {
            Self::Startup => "startup",
            Self::Scroll => "scroll",
            Self::Nav => "nav",
            Self::Resize => "resize",
            Self::Idle => "idle",
        }
    }
}

#[derive(Debug, Clone)]
pub struct FrameRow {
    pub phase: &'static str,
    pub frame: usize,
    pub cpu_ms: f64,
    pub interval_ms: f64,
    pub cells: usize,
    pub vis_rows: usize,
    pub win_w: f32,
    pub win_h: f32,
}

pub struct Bench {
    pub scenario: Scenario,
    pub started: Instant,
    pub frames: usize,
    pub rows: Vec<FrameRow>,
    pub first_ui_ms: Option<f64>,
    pub ready_ms: Option<f64>,
}

impl Bench {
    pub fn new(scenario: Scenario, started: Instant) -> Self {
        Self {
            scenario,
            started,
            frames: 0,
            rows: Vec::new(),
            first_ui_ms: None,
            ready_ms: None,
        }
    }

    pub fn csv(&self) -> String {
        let mut s = String::from("phase,frame,cpu_ms,interval_ms,cells,vis_rows,win_w,win_h\n");
        for r in &self.rows {
            let _ = writeln!(
                s,
                "{},{},{:.4},{:.4},{},{},{:.0},{:.0}",
                r.phase, r.frame, r.cpu_ms, r.interval_ms, r.cells, r.vis_rows, r.win_w, r.win_h
            );
        }
        s
    }
}
