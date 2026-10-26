use poc_ui::model::grid::{Cell, ComparisonSource, RowSource};

#[test]
fn comparison_adapter_exposes_all_rows_and_preserves_exact_types() {
    let source = ComparisonSource(1_000_000);
    assert_eq!(source.row_count(), 1_000_000);
    assert_eq!(source.col_count(), 20);
    assert_eq!(source.column_name(5), "TIMESTAMP");
    assert_eq!(source.column_name(6), "LONG_TEXT");
    assert_eq!(source.cell(7, 3), Cell::Null);
    assert_eq!(source.cell(42, 1), Cell::Text("ลูกค้า 42".into()));
    assert_eq!(
        source.cell(999_999, 2),
        Cell::Number("7918992081.000999999".into())
    );
}
