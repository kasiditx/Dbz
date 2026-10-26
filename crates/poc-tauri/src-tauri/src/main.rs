#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod query;
use serde::Serialize;
use std::{io::Write, sync::Mutex, time::Instant};
use tauri::State;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct Config {
    rows: usize,
    large_sql: bool,
    measure: bool,
    seconds: f64,
}
struct Diagnostics {
    config: Config,
    started: Instant,
    events: Mutex<Option<std::fs::File>>,
    frames: Mutex<Option<std::fs::File>>,
}

#[tauri::command]
fn configuration(state: State<'_, Diagnostics>) -> Config {
    state.config.clone()
}
#[tauri::command]
async fn run_query(outcome: String, state: State<'_, query::Query>) -> Result<String, String> {
    state
        .run(&outcome)
        .await
        .map(str::to_owned)
        .map_err(str::to_owned)
}
#[tauri::command]
fn cancel_query(state: State<'_, query::Query>) {
    state.cancel();
}
#[tauri::command]
fn ready(state: State<'_, Diagnostics>) {
    println!(
        "READY pid={} ready_ms={:.2}",
        std::process::id(),
        state.started.elapsed().as_secs_f64() * 1000.0
    );
}
#[tauri::command]
fn snapshot(value: serde_json::Value, state: State<'_, Diagnostics>) -> Result<(), String> {
    if !state.config.measure {
        return Ok(());
    }
    let line = serde_json::to_string(&value).map_err(|_| "Invalid snapshot")?;
    if line.len() > 16_384 {
        return Err("Snapshot too large".into());
    }
    let mut file = state.events.lock().map_err(|_| "Diagnostic lock failed")?;
    if let Some(file) = file.as_mut() {
        writeln!(file, "{line}").map_err(|_| "Diagnostic write failed")?;
        file.flush().map_err(|_| "Diagnostic flush failed")?;
    }
    Ok(())
}
#[tauri::command]
fn frames(value: Vec<serde_json::Value>, state: State<'_, Diagnostics>) -> Result<(), String> {
    if !state.config.measure {
        return Ok(());
    }
    if value.len() > 20_000 {
        return Err("Too many frame samples".into());
    }
    let mut file = state.frames.lock().map_err(|_| "Diagnostic lock failed")?;
    if let Some(file) = file.as_mut() {
        for row in value {
            writeln!(file, "{row}").map_err(|_| "Frame write failed")?;
        }
        file.flush().map_err(|_| "Frame flush failed")?;
    }
    Ok(())
}
fn output(path: Option<&str>) -> std::io::Result<Option<std::fs::File>> {
    path.map(std::fs::File::create).transpose()
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let started = Instant::now();
    let args: Vec<String> = std::env::args().collect();
    let get = |key: &str| {
        args.iter()
            .position(|v| v == key)
            .and_then(|i| args.get(i + 1))
            .map(String::as_str)
    };
    let rows = get("--rows").unwrap_or("100000").parse::<usize>()?;
    if ![1000, 100000, 1000000].contains(&rows) {
        return Err("Use --rows 1000, 100000 or 1000000".into());
    }
    let config = Config {
        rows,
        large_sql: args.iter().any(|v| v == "--large-sql"),
        measure: args.iter().any(|v| v == "--measure"),
        seconds: get("--seconds").unwrap_or("0").parse()?,
    };
    tauri::Builder::default()
        .manage(Diagnostics {
            config,
            started,
            events: Mutex::new(output(get("--event-log"))?),
            frames: Mutex::new(output(get("--out"))?),
        })
        .manage(query::Query::default())
        .invoke_handler(tauri::generate_handler![
            configuration,
            run_query,
            cancel_query,
            ready,
            snapshot,
            frames
        ])
        .run(tauri::generate_context!())?;
    Ok(())
}
