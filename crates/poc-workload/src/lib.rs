//! Shared, synthetic-only T-004a workload. Never connects to a database.
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contract {
    pub columns: usize,
    pub row_height: f32,
    pub column_width: f32,
    pub wide_column_width: f32,
    pub wide_column: usize,
    pub overscan: usize,
    pub tabs: usize,
    pub query_delay_ms: u64,
    pub large_sql_repeats: usize,
}

pub fn contract() -> Contract {
    serde_json::from_str(include_str!("../../../benchmarks/t004a/workload.json"))
        .expect("checked-in workload contract must be valid")
}

pub fn sql(large: bool) -> String {
    include_str!("../../../benchmarks/t004a/query.sql").repeat(if large {
        contract().large_sql_repeats
    } else {
        1
    })
}

/// Exact decimal strings; O(1) cells, no dataset allocation. Coordinates are zero-based.
pub fn cell(row: usize, col: usize) -> Option<String> {
    match col {
        0 => Some(row.to_string()),
        1 => Some(format!("ลูกค้า {row}")),
        2 => Some(format!("{}.{:09}", row * 7919, row % 1_000_000_000)),
        3 if row.is_multiple_of(7) => None,
        3 => Some(["OPEN", "PAID", "VOID"][row % 3].into()),
        4 => Some(format!("หมายเหตุยาว {row} ").repeat(30)),
        5 => Some(format!(
            "2026-01-15T{:02}:{:02}:{:02}.000Z",
            row / 3600 % 24,
            row / 60 % 60,
            row % 60
        )),
        6 => Some(format!("Long Unicode Ω 日本語 {row} ").repeat(30)),
        n => Some(format!("r{row}c{n}")),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn timestamp_and_long_text_match_the_contract() {
        assert_eq!(
            super::cell(42, 5).as_deref(),
            Some("2026-01-15T00:00:42.000Z")
        );
        assert!(super::cell(42, 6).unwrap().chars().count() > 300);
        assert_eq!(super::cell(7, 3), None);
    }
}
