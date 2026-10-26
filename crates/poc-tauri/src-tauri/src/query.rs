//! One bounded mock job, with acknowledged cancellation; no SQL execution.
use std::sync::{
    Mutex,
    atomic::{AtomicBool, Ordering},
};

#[derive(Default)]
pub struct Query {
    active: Mutex<bool>,
    cancel: AtomicBool,
}

impl Query {
    pub async fn run(&self, outcome: &str) -> Result<&'static str, &'static str> {
        if !["success", "empty", "error"].contains(&outcome) {
            return Err("Unknown mock outcome");
        }
        {
            let mut active = self.active.lock().map_err(|_| "Mock state unavailable")?;
            if *active {
                return Err("A mock query is already running");
            }
            *active = true;
            self.cancel.store(false, Ordering::Relaxed);
        }
        let started = std::time::Instant::now();
        let delay = std::time::Duration::from_millis(poc_workload::contract().query_delay_ms);
        let result = loop {
            if self.cancel.load(Ordering::Relaxed) {
                break Ok("cancelled");
            }
            if started.elapsed() >= delay {
                break match outcome {
                    "empty" => Ok("empty"),
                    "error" => Err("Simulated query error. Retry or choose Success."),
                    _ => Ok("done"),
                };
            }
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        };
        *self.active.lock().map_err(|_| "Mock state unavailable")? = false;
        result
    }
    pub fn cancel(&self) {
        self.cancel.store(true, Ordering::Relaxed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn cancellation_is_acknowledged_and_a_second_job_is_rejected() {
        let query = Query::default();
        let run = query.run("success");
        let control = async {
            tokio::time::sleep(std::time::Duration::from_millis(20)).await;
            assert_eq!(
                query.run("success").await,
                Err("A mock query is already running")
            );
            query.cancel();
        };
        let (result, ()) = tokio::join!(run, control);
        assert_eq!(result, Ok("cancelled"));
        assert_eq!(query.run("empty").await, Ok("empty"));
    }
    #[tokio::test]
    async fn validation_and_failure_do_not_leave_the_job_busy() {
        let query = Query::default();
        assert_eq!(query.run("invalid").await, Err("Unknown mock outcome"));
        assert!(query.run("error").await.is_err());
        assert_eq!(query.run("success").await, Ok("done"));
    }
}
