//! Live Oracle test. Ignored by default so it shows as `ignored`, never as a false pass.
//! Run: cargo test --test live_oracle -- --ignored (needs DBZ_ORACLE_* env). Missing env => FAIL.
use poc_oracle::config::from_env;
use poc_oracle::query::{Bind, Value, run_select};
use poc_oracle::real::OracleSession;
use std::time::Duration;

#[test]
#[ignore = "NOT TESTED: needs authorized live Oracle"]
fn live_select_bind_exact_number() {
    let cfg = match from_env(|k| std::env::var(k).ok()) {
        Ok(c) => c,
        Err(e) => panic!("live test requested but not runnable: {e}"),
    };
    let s = OracleSession::connect(&cfg, Duration::from_secs(10)).expect("connect");
    let p = run_select(
        &s,
        "select :1 as n from dual",
        &Bind::Number("12345678901234567890.123456789".into()),
        100,
    )
    .expect("query");
    assert_eq!(
        p.rows[0][0],
        Value::Number("12345678901234567890.123456789".into())
    );
    s.close().expect("close");
}
