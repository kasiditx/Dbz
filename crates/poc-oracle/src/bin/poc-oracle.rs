//! CLI POC: test_connection, SELECT :bind, typed fetch, close.
//! Env: DBZ_ORACLE_{HOST,PORT,SERVICE,USER,PASSWORD}. Prints NOT TESTED when env absent.
use poc_oracle::config::from_env;
use poc_oracle::error::PocError;
use poc_oracle::query::{Bind, run_select};
use poc_oracle::real::OracleSession;
use std::time::Duration;

fn main() {
    let cfg = match from_env(|k| std::env::var(k).ok()) {
        Ok(c) => c,
        Err(PocError::MissingEnv(names)) => {
            println!(
                "NOT TESTED: live Oracle (missing env: {})",
                names.join(", ")
            );
            return;
        }
        Err(e) => {
            eprintln!("config error: {e}");
            std::process::exit(2);
        }
    };
    let s = match OracleSession::connect(&cfg, Duration::from_secs(10)) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("connect failed: {e}");
            std::process::exit(1);
        }
    };
    match s.session_id() {
        Ok(id) => println!("connected, session id {id}"),
        Err(e) => eprintln!("session id unavailable: {e}"),
    }
    let page = run_select(
        &s,
        "select :1 as n, 'x' as t, cast(null as varchar2(1)) as nul from dual",
        &Bind::Number("12345678901234567890.123456789".into()),
        100,
    );
    match page {
        Ok(p) => println!("{p:#?}"),
        Err(e) => eprintln!("query failed: {e}"),
    }
    if let Err(e) = s.close() {
        eprintln!("close failed: {e}");
    }
}
