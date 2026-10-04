use std::time::{Duration, SystemTime, UNIX_EPOCH};

use metrics::{counter, gauge, histogram};

pub const BASE_GOV_REQUEST_DURATION_SECONDS: &str = "scraper_base_gov_request_duration_seconds";

#[derive(Debug, Clone, Copy)]
pub enum RequestKind {
    Page,
    Details,
}

impl RequestKind {
    fn as_str(self) -> &'static str {
        match self {
            RequestKind::Page => "page",
            RequestKind::Details => "details",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ContractFailure {
    /// Gave up fetching the contract details after all retries
    Fetch,
    /// Fetched the contract but couldn't save it
    Save,
}

impl ContractFailure {
    fn as_str(self) -> &'static str {
        match self {
            ContractFailure::Fetch => "fetch",
            ContractFailure::Save => "save",
        }
    }
}

fn unix_now() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64()
}

fn register_counters() {
    for kind in [RequestKind::Page, RequestKind::Details] {
        for result in ["ok", "error"] {
            counter!("scraper_base_gov_requests_total", "kind" => kind.as_str(), "result" => result)
                .increment(0);
        }
    }
    for reason in [ContractFailure::Fetch, ContractFailure::Save] {
        counter!("scraper_contracts_failed_total", "reason" => reason.as_str()).increment(0);
    }
    for result in ["completed", "aborted"] {
        counter!("scraper_runs_total", "result" => result).increment(0);
    }
    counter!("scraper_contracts_saved_total").increment(0);
}

pub fn run_started() {
    register_counters();
    gauge!("scraper_running").set(1.0);
    gauge!("scraper_last_run_start_timestamp_seconds").set(unix_now());
}

pub fn run_finished(completed: bool, duration: Duration) {
    let now = unix_now();
    let result = if completed { "completed" } else { "aborted" };

    gauge!("scraper_running").set(0.0);
    gauge!("scraper_last_run_duration_seconds").set(duration.as_secs_f64());
    counter!("scraper_runs_total", "result" => result).increment(1);
    if completed {
        gauge!("scraper_last_success_timestamp_seconds").set(now);
    }
}

pub fn next_run_in(delay: Duration) {
    gauge!("scraper_next_run_timestamp_seconds").set(unix_now() + delay.as_secs_f64());
}

pub fn request(kind: RequestKind, ok: bool, duration: Duration) {
    let result = if ok { "ok" } else { "error" };
    counter!("scraper_base_gov_requests_total", "kind" => kind.as_str(), "result" => result)
        .increment(1);
    histogram!(BASE_GOV_REQUEST_DURATION_SECONDS, "kind" => kind.as_str())
        .record(duration.as_secs_f64());
}

pub fn page_progress(current_page: usize, total_pages: Option<usize>) {
    gauge!("scraper_current_page").set(current_page as f64);
    if let Some(total_pages) = total_pages {
        gauge!("scraper_total_pages").set(total_pages as f64);
    }
}

pub fn contract_saved() {
    counter!("scraper_contracts_saved_total").increment(1);
}

pub fn contract_failed(reason: ContractFailure) {
    counter!("scraper_contracts_failed_total", "reason" => reason.as_str()).increment(1);
}
