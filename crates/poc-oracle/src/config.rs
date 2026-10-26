use crate::error::PocError;
use std::fmt;

/// Secret string; Debug/Display never reveal it.
#[derive(Clone)]
pub struct Secret(String);

impl Secret {
    pub fn new(v: impl Into<String>) -> Self {
        Secret(v.into())
    }
    pub fn expose(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for Secret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Secret(<redacted>)")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectTarget {
    host: String,
    port: u16,
    service_name: String,
}

impl ConnectTarget {
    /// Service Name target only (SID is not interchangeable, OR-002 later).
    pub fn new(host: &str, port: u16, service_name: &str) -> Result<Self, PocError> {
        check_part("host", host)?;
        check_part("service name", service_name)?;
        if port == 0 {
            return Err(PocError::InvalidConfig("port must be 1-65535".into()));
        }
        Ok(ConnectTarget {
            host: host.into(),
            port,
            service_name: service_name.into(),
        })
    }
    /// EZConnect `host:port/service`.
    pub fn connect_string(&self) -> String {
        format!("{}:{}/{}", self.host, self.port, self.service_name)
    }
}

fn check_part(what: &str, v: &str) -> Result<(), PocError> {
    let bad = v.is_empty()
        || v.chars()
            .any(|c| c.is_whitespace() || matches!(c, '/' | ':' | '(' | ')' | '\\' | '"' | '\''));
    if bad {
        return Err(PocError::InvalidConfig(format!(
            "{what} empty or has forbidden characters"
        )));
    }
    Ok(())
}

#[derive(Debug, Clone)]
pub struct PocConfig {
    pub target: ConnectTarget,
    pub user: String,
    pub password: Secret,
}

pub const ENV_HOST: &str = "DBZ_ORACLE_HOST";
pub const ENV_PORT: &str = "DBZ_ORACLE_PORT";
pub const ENV_SERVICE: &str = "DBZ_ORACLE_SERVICE";
pub const ENV_USER: &str = "DBZ_ORACLE_USER";
pub const ENV_PASSWORD: &str = "DBZ_ORACLE_PASSWORD";

/// Build config from an env lookup fn. Port defaults to 1521.
pub fn from_env(get: impl Fn(&str) -> Option<String>) -> Result<PocConfig, PocError> {
    let missing: Vec<String> = [ENV_HOST, ENV_SERVICE, ENV_USER, ENV_PASSWORD]
        .into_iter()
        .filter(|k| get(k).is_none_or(|v| v.is_empty()))
        .map(String::from)
        .collect();
    if !missing.is_empty() {
        return Err(PocError::MissingEnv(missing));
    }
    let port = match get(ENV_PORT) {
        None => 1521,
        Some(p) => p
            .parse::<u16>()
            .map_err(|_| PocError::InvalidConfig(format!("{ENV_PORT} not a valid port")))?,
    };
    let target = ConnectTarget::new(&get(ENV_HOST).unwrap(), port, &get(ENV_SERVICE).unwrap())?;
    Ok(PocConfig {
        target,
        user: get(ENV_USER).unwrap(),
        password: Secret::new(get(ENV_PASSWORD).unwrap()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn env(pairs: &[(&str, &str)]) -> impl Fn(&str) -> Option<String> {
        let m: HashMap<String, String> = pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        move |k| m.get(k).cloned()
    }

    #[test]
    fn connect_string_is_host_port_service() {
        let t = ConnectTarget::new("db.example", 1521, "ORCLPDB1").unwrap();
        assert_eq!(t.connect_string(), "db.example:1521/ORCLPDB1");
    }

    #[test]
    fn rejects_descriptor_injection_chars() {
        for bad in ["a b", "a/b", "a:b", "a(b", "a)b", ""] {
            assert!(ConnectTarget::new(bad, 1521, "S").is_err(), "host {bad:?}");
            assert!(
                ConnectTarget::new("h", 1521, bad).is_err(),
                "service {bad:?}"
            );
        }
    }

    #[test]
    fn rejects_port_zero() {
        assert!(ConnectTarget::new("h", 0, "S").is_err());
    }

    #[test]
    fn secret_debug_is_redacted() {
        let s = format!("{:?}", Secret::new("hunter2"));
        assert!(!s.contains("hunter2"));
    }

    #[test]
    fn config_debug_never_leaks_password() {
        let cfg = from_env(env(&[
            (ENV_HOST, "h"),
            (ENV_SERVICE, "S"),
            (ENV_USER, "u"),
            (ENV_PASSWORD, "hunter2"),
        ]))
        .unwrap();
        assert!(!format!("{cfg:?}").contains("hunter2"));
        assert_eq!(cfg.password.expose(), "hunter2");
    }

    #[test]
    fn from_env_defaults_port_1521() {
        let cfg = from_env(env(&[
            (ENV_HOST, "h"),
            (ENV_SERVICE, "S"),
            (ENV_USER, "u"),
            (ENV_PASSWORD, "p"),
        ]))
        .unwrap();
        assert_eq!(cfg.target.connect_string(), "h:1521/S");
    }

    #[test]
    fn from_env_reports_all_missing_names_not_values() {
        let err = from_env(env(&[(ENV_HOST, "h")])).unwrap_err();
        match err {
            PocError::MissingEnv(names) => {
                assert_eq!(names, vec![ENV_SERVICE, ENV_USER, ENV_PASSWORD]);
            }
            other => panic!("unexpected {other:?}"),
        }
    }

    #[test]
    fn from_env_rejects_bad_port() {
        let err = from_env(env(&[
            (ENV_HOST, "h"),
            (ENV_PORT, "99999"),
            (ENV_SERVICE, "S"),
            (ENV_USER, "u"),
            (ENV_PASSWORD, "p"),
        ]))
        .unwrap_err();
        assert!(matches!(err, PocError::InvalidConfig(_)));
    }
}
