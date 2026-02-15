// Resolve OpenClaw config path and read gateway.auth.token so we can load
// the dashboard with ?token= for WebSocket auth.

use std::env;
use std::fs;
use std::path::PathBuf;

/// Resolve path to openclaw.json. Uses OPENCLAW_CONFIG_PATH, else OPENCLAW_STATE_DIR/openclaw.json,
/// else $HOME/.openclaw/openclaw.json. On Windows set OPENCLAW_CONFIG_PATH or OPENCLAW_STATE_DIR if HOME is not set.
fn default_config_path() -> Option<PathBuf> {
    if let Ok(p) = env::var("OPENCLAW_CONFIG_PATH") {
        let trimmed = p.trim();
        if !trimmed.is_empty() {
            return Some(PathBuf::from(trimmed));
        }
    }
    let state_dir = env::var("OPENCLAW_STATE_DIR")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .map(PathBuf::from)
        .or_else(|| {
            env::var("HOME").ok().map(|home| {
                let mut p = PathBuf::from(home);
                p.push(".openclaw");
                p
            })
        })?;
    let mut p = state_dir;
    p.push("openclaw.json");
    Some(p)
}

#[derive(serde::Deserialize)]
struct GatewayAuth {
    token: Option<String>,
}

#[derive(serde::Deserialize)]
struct Gateway {
    auth: Option<GatewayAuth>,
}

#[derive(serde::Deserialize)]
struct OpenClawConfig {
    gateway: Option<Gateway>,
}

/// Returns the dashboard URL with token in query if available.
/// Base URL is http://127.0.0.1:18789/ (default gateway port).
pub fn dashboard_url_with_token() -> String {
    const BASE: &str = "http://127.0.0.1:18789/";
    let path = match default_config_path() {
        Some(p) => p,
        None => return BASE.to_string(),
    };
    let contents = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => return BASE.to_string(),
    };
    let config: OpenClawConfig = match serde_json::from_str(&contents) {
        Ok(c) => c,
        Err(_) => return BASE.to_string(),
    };
    let token = config
        .gateway
        .and_then(|g| g.auth)
        .and_then(|a| a.token)
        .map(|t| t.trim().to_string())
        .filter(|t| !t.is_empty());
    match token {
        Some(t) => {
            let encoded = urlencoding::encode(&t);
            format!("{}?token={}", BASE, encoded)
        }
        None => BASE.to_string(),
    }
}
