//! Rate limiting middleware — fixed window per client IP
//! (Rate limit: cửa sổ cố định theo IP client, in-memory)

use axum::{
    extract::{ConnectInfo, Request},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::Instant;

/// Cấu hình rate limit
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    /// Số request tối đa trong cửa sổ — 0 = tắt
    pub max_requests: usize,
    /// Độ dài cửa sổ (giây)
    pub window_secs: u64,
}

struct WindowState {
    started: Instant,
    count: usize,
}

/// Bộ đếm in-memory — ponytail: đơn giản theo IP, đủ cho registry private;
/// nâng lên per-token + hậu trường Redis khi cần scale
/// Windows of idleness before an untouched client entry is dropped.
///
/// Ten windows: long enough that a realistic burst pattern is unaffected, short
/// enough that a registry serving many one-off IPs does not accumulate them.
/// (Mười cửa sổ im lặng trước khi xoá entry — đủ dài để không ảnh hưởng burst
/// thật, đủ ngắn để không tích entry.)
const SWEEP_WINDOWS: u64 = 10;

pub struct RateLimiter {
    config: RateLimitConfig,
    windows: Mutex<HashMap<String, WindowState>>,
    last_sweep: Mutex<Instant>,
}

impl RateLimiter {
    pub fn new(config: RateLimitConfig) -> Self {
        Self {
            config,
            windows: Mutex::new(HashMap::new()),
            last_sweep: Mutex::new(Instant::now()),
        }
    }

    /// Kiểm tra + tăng bộ đếm cho client; false = vượt giới hạn
    pub fn allow(&self, client: &str) -> bool {
        let mut windows = self
            .windows
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let now = Instant::now();
        let mut last_sweep = self
            .last_sweep
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let state = windows.entry(client.to_string()).or_insert(WindowState {
            started: now,
            count: 0,
        });
        if state.started.elapsed().as_secs() >= self.config.window_secs {
            state.started = now;
            state.count = 0;
        }
        state.count += 1;
        let allowed = state.count <= self.config.max_requests;
        // Drop windows that are long idle.
        //
        // The map never shrank, so every IP that ever hit the registry kept an
        // entry for the process lifetime — unbounded memory on a long-running
        // private registry. Sweeping here (rather than on a timer) needs no
        // background task and runs at most once per window length.
        // (Map không bao giờ co lại nên mỗi IP từng gọi giữ một entry suốt đời
        // process — tốn bộ nhớ vô hạn. Quét tại đây không cần task nền.)
        let sweep_after = self.config.window_secs.saturating_mul(SWEEP_WINDOWS);
        if last_sweep.elapsed().as_secs() >= sweep_after {
            windows.retain(|_, window| window.started.elapsed().as_secs() < sweep_after);
            *last_sweep = now;
        }
        allowed
    }

    /// Number of client windows currently tracked; used by tests to prove the
    /// sweep actually releases memory.
    /// (Số client window đang theo dõi; test dùng để chứng minh quét thật sự giải
    /// phóng bộ nhớ.)
    #[cfg(test)]
    fn tracked_clients(&self) -> usize {
        self.windows
            .lock()
            .map(|windows| windows.len())
            .unwrap_or(0)
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;

/// Axum middleware gắn rate limit
pub async fn rate_limit_middleware(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let limiter = request
        .extensions()
        .get::<Arc<RateLimiter>>()
        .cloned()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    if !limiter.allow(&addr.ip().to_string()) {
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }
    Ok(next.run(request).await)
}
