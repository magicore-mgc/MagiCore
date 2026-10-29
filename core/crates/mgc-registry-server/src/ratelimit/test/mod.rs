//! Rate-limiter tests — test giới hạn tần suất.
//! The limiter decides whether a caller may proceed, so both directions matter:
//! a limit that never fires is useless, and a map that never shrinks grows
//! without bound on a long-running private registry.
//! (Limiter quyết định caller có được đi tiếp, nên cả hai chiều đều quan trọng:
//! giới hạn không bao giờ bắn là vô dụng, và map không co lại sẽ phình vô hạn.)

use super::{RateLimitConfig, RateLimiter};

fn limiter(max_requests: usize, window_secs: u64) -> RateLimiter {
    RateLimiter::new(RateLimitConfig {
        max_requests,
        window_secs,
    })
}

/// The limit fires exactly at `max_requests + 1`.
/// (Giới hạn bắn đúng tại `max_requests + 1`.)
#[test]
fn the_limit_fires_after_the_budget_is_spent() {
    let limiter = limiter(3, 1);
    assert!(limiter.allow("10.0.0.1"));
    assert!(limiter.allow("10.0.0.1"));
    assert!(limiter.allow("10.0.0.1"));
    assert!(
        !limiter.allow("10.0.0.1"),
        "the fourth request in the window must be refused"
    );
}

/// Each client gets its own budget — one noisy caller cannot starve another.
/// (Mỗi client có ngân sách riêng — caller ồn ào không bóp nghẹt client khác.)
#[test]
fn one_client_exhausting_its_budget_does_not_affect_another() {
    let limiter = limiter(1, 60);
    assert!(limiter.allow("10.0.0.1"));
    assert!(!limiter.allow("10.0.0.1"));
    assert!(
        limiter.allow("10.0.0.2"),
        "a different client must start with a full budget"
    );
}

/// Every distinct client is tracked, so memory grows with distinct IPs.
///
/// This is the shape the sweep exists for; asserting it keeps the test honest
/// about what the map holds.
/// (Mỗi client phân biệt được theo dõi nên bộ nhớ tăng theo số IP phân biệt. Đây
/// là hình dạng mà việc quét tồn tại để xử lý.)
#[test]
fn distinct_clients_are_tracked_separately() {
    let limiter = limiter(5, 60);
    for index in 0..50 {
        limiter.allow(&format!("10.0.1.{index}"));
    }
    assert_eq!(
        limiter.tracked_clients(),
        50,
        "each distinct client needs a window until the sweep runs"
    );
}

/// A window older than the sweep horizon is released.
///
/// `window_secs` drives both the budget window and the sweep horizon, so a
/// 1-second window makes the sweep observable without a long sleep.
/// (`window_secs` điều khiển cả cửa sổ ngân sách và mốc quét, nên cửa sổ 1 giây
/// làm việc quét quan sát được mà không cần chờ lâu.)
#[test]
fn idle_client_windows_are_released() {
    let limiter = limiter(5, 1);
    for index in 0..20 {
        limiter.allow(&format!("10.0.2.{index}"));
    }
    assert_eq!(limiter.tracked_clients(), 20);

    // The sweep horizon is `window_secs * 10`; waiting past it lets the next call
    // drop every idle entry.
    // (Mốc quét là `window_secs * 10`; chờ qua mốc đó để lần gọi sau dọn sạch.)
    std::thread::sleep(std::time::Duration::from_millis(11_000));
    limiter.allow("192.0.2.1");

    assert!(
        limiter.tracked_clients() < 20,
        "idle windows must be released, still tracking {}",
        limiter.tracked_clients()
    );
}
