use std::{io::ErrorKind, net::IpAddr, path::Path, sync::Arc};

use anyhow::Context;
use axum::{
    extract::{Request, State},
    http::header::USER_AGENT,
    middleware::Next,
    response::{IntoResponse, Response},
};
use serde::Deserialize;
use tracing::{debug, info};

use crate::{error::AppError, extractors::ClientIp};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Trigger {
    Ip(IpAddr),
    UserAgent(String),
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Entry {
    trigger: Trigger,
    reason: String,
}

#[derive(Debug, Default)]
pub struct Blocklist {
    entries: Vec<Entry>,
}

impl Blocklist {
    pub fn load(path: &Path) -> anyhow::Result<Self> {
        let path_display = path.display();
        let content = match std::fs::read_to_string(path) {
            Ok(content) => content,
            Err(e) if e.kind() == ErrorKind::NotFound => {
                info!("No blocklist found at {path_display}, not blocking anyone");
                return Ok(Self::default());
            }
            Err(e) => {
                return Err(e).with_context(|| format!("Failed to read blocklist {path_display}"));
            }
        };

        let entries: Vec<Entry> = serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse blocklist {path_display}"))?;

        let len = entries.len();

        info!("Loaded {len} blocklist entries from {path_display}");

        Ok(Self { entries })
    }

    fn find(&self, ip: IpAddr, user_agent: Option<&str>) -> Option<&Entry> {
        self.entries.iter().find(|entry| match &entry.trigger {
            Trigger::Ip(blocked) => *blocked == ip,
            Trigger::UserAgent(blocked) => user_agent == Some(blocked.as_str()),
        })
    }
}

pub async fn blocklist_layer(
    State(blocklist): State<Arc<Blocklist>>,
    ClientIp(ip): ClientIp,
    request: Request,
    next: Next,
) -> Response {
    let user_agent = request
        .headers()
        .get(USER_AGENT)
        .and_then(|v| v.to_str().ok());

    if let Some(entry) = blocklist.find(ip, user_agent) {
        debug!(ip = %ip, user_agent, trigger = ?entry.trigger, "Blocked request");
        return AppError::Blocked(entry.reason.clone()).into_response();
    }

    next.run(request).await
}
