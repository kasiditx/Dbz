//! T-003 POC entry. Args: --rows N --cols N --light --bench <startup|scroll|nav|resize|idle>
//!   --out file.csv --screenshot file.png
use poc_ui::app::{Options, PocApp};
use poc_ui::bench::Scenario;
use std::time::Instant;

fn main() -> eframe::Result {
    let started = Instant::now();
    let args: Vec<String> = std::env::args().skip(1).collect();
    let get = |k: &str| {
        args.iter()
            .position(|a| a == k)
            .and_then(|i| args.get(i + 1))
            .cloned()
    };
    let opts = Options {
        rows: get("--rows")
            .and_then(|v| v.parse().ok())
            .unwrap_or(100_000),
        cols: get("--cols").and_then(|v| v.parse().ok()).unwrap_or(20),
        dark: !args.iter().any(|a| a == "--light"),
        bench: get("--bench").and_then(|v| Scenario::parse(&v)),
        out: get("--out"),
        screenshot: get("--screenshot"),
        started,
    };
    let native = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 800.0])
            .with_min_inner_size([640.0, 400.0])
            .with_title("DB Client POC (T-003)"),
        ..Default::default()
    };
    eframe::run_native(
        "dbz-ui-poc",
        native,
        Box::new(|cc| Ok(Box::new(PocApp::new(cc, opts)))),
    )
}
