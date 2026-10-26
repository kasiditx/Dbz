//! Adapter to oracledb 26.0.0-beta.4 (blocking API).
//! Compiles and error mapping is unit-tested; query/connect paths are NOT TESTED against live Oracle.
use crate::config::PocConfig;
use crate::error::{PocError, redact};
use crate::query::{Bind, ResultPage, SelectBackend, Value};
use oracledb::{ErrorKind, OracleNumber};
use std::str::FromStr;
use std::time::Duration;

/// Map driver error kind to POC error. Message text is redacted of `secrets`.
pub fn map_kind(kind: &ErrorKind, display: &str, secrets: &[&str]) -> PocError {
    match kind {
        ErrorKind::DbError(e) => PocError::Db {
            code: e.code() as u32,
            message: redact(e.message(), secrets),
        },
        ErrorKind::CallTimeoutExceeded => PocError::Timeout,
        ErrorKind::DeadConnection => PocError::DeadConnection,
        _ => PocError::Other(redact(display, secrets)),
    }
}

pub struct OracleSession {
    conn: oracledb::Connection,
    secrets: Vec<String>,
}

impl OracleSession {
    /// Connect (blocking; call from a worker thread, never the UI thread).
    pub fn connect(cfg: &PocConfig, call_timeout: Duration) -> Result<Self, PocError> {
        let secrets = vec![cfg.password.expose().to_string()];
        let sref: Vec<&str> = secrets.iter().map(String::as_str).collect();
        let config = oracledb::Config::default()
            .set_credentials(&cfg.user, cfg.password.expose())
            .set_connect_string(&cfg.target.connect_string())
            .map_err(|e| map_kind(e.kind(), &e.to_string(), &sref))?;
        let conn =
            oracledb::connect(config).map_err(|e| map_kind(e.kind(), &e.to_string(), &sref))?;
        conn.set_call_timeout(Some(call_timeout))
            .map_err(|e| map_kind(e.kind(), &e.to_string(), &sref))?;
        Ok(OracleSession { conn, secrets })
    }

    fn map(&self, e: oracledb::Error) -> PocError {
        let s: Vec<&str> = self.secrets.iter().map(String::as_str).collect();
        map_kind(e.kind(), &e.to_string(), &s)
    }

    pub fn ping(&self) -> Result<(), PocError> {
        self.conn.ping().map_err(|e| self.map(e))
    }

    pub fn session_id(&self) -> Result<usize, PocError> {
        self.conn.session_id().map_err(|e| self.map(e))
    }

    pub fn close(mut self) -> Result<(), PocError> {
        self.conn.close().map_err(|e| self.map(e))
    }
}

impl SelectBackend for OracleSession {
    fn query(&self, sql: &str, bind: &Bind, max_rows: usize) -> Result<ResultPage, PocError> {
        let num;
        let txt;
        let param: &dyn oracledb::ToDbValue = match bind {
            Bind::Number(n) => {
                num = OracleNumber::from_str(n)
                    .map_err(|_| PocError::InvalidConfig("bind is not a valid NUMBER".into()))?;
                &num
            }
            Bind::Text(t) => {
                txt = t.clone();
                &txt
            }
        };
        let cursor = self.conn.query(sql, &[param]).map_err(|e| self.map(e))?;
        let columns: Vec<String> = cursor
            .columns()
            .iter()
            .map(|c| c.name().to_string())
            .collect();
        let is_number: Vec<bool> = cursor
            .columns()
            .iter()
            .map(|c| c.data_type().to_ascii_uppercase().contains("NUMBER"))
            .collect();
        let types: Vec<String> = cursor.columns().iter().map(|c| c.data_type()).collect();
        let mut rows = Vec::new();
        // Fetch max_rows + 1 only: bounded, no full-table fetch.
        for row in cursor.take(max_rows + 1) {
            let row = row.map_err(|e| self.map(e))?;
            let mut out = Vec::with_capacity(columns.len());
            for i in 0..columns.len() {
                let v = if is_number[i] {
                    row.get::<Option<OracleNumber>>(i)
                        .map(|o| o.map_or(Value::Null, |n| Value::Number(n.to_string())))
                } else {
                    row.get::<Option<String>>(i)
                        .map(|o| o.map_or(Value::Null, Value::Text))
                };
                out.push(v.unwrap_or_else(|_| Value::Unsupported(types[i].clone())));
            }
            rows.push(out);
        }
        Ok(ResultPage {
            columns,
            rows,
            has_more: false,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timeout_kind_maps_to_timeout() {
        assert_eq!(
            map_kind(&ErrorKind::CallTimeoutExceeded, "x", &[]),
            PocError::Timeout
        );
    }

    #[test]
    fn dead_connection_kind_maps() {
        assert_eq!(
            map_kind(&ErrorKind::DeadConnection, "x", &[]),
            PocError::DeadConnection
        );
    }

    #[test]
    fn other_kind_redacts_secret_in_message() {
        let e = map_kind(
            &ErrorKind::NoConnectString,
            "bad hunter2 here",
            &["hunter2"],
        );
        assert_eq!(e, PocError::Other("bad *** here".into()));
    }
}
