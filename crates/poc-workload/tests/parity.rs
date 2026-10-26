use poc_workload::cell;
use serde::Deserialize;

#[derive(Deserialize)]
struct Example {
    row: usize,
    col: usize,
    value: Option<String>,
}

#[test]
fn shared_language_neutral_examples_match() {
    let examples: Vec<Example> =
        serde_json::from_str(include_str!("../../../benchmarks/t004a/cells.json")).unwrap();
    for e in examples {
        assert_eq!(cell(e.row, e.col), e.value, "row {} col {}", e.row, e.col);
    }
}
