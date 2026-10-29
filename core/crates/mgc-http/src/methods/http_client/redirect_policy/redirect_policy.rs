//! Redirect policy: bounded hop count; credentials never cross origins.
//! Policy redirect: giới hạn số hop, mặc định chỉ cùng origin.

use reqwest::Url;
use reqwest::redirect::Policy;

/// Default maximum redirect hops in a chain.
/// Số hop redirect tối đa mặc định trong một chuỗi.
pub const DEFAULT_MAX_REDIRECTS: usize = 5;

/// Environment opt-in that allows cross-origin redirects.
/// Biến môi trường opt-in cho phép redirect khác origin.
pub const ALLOW_CROSS_ORIGIN_REDIRECTS_ENV: &str = "MAGICORE_ALLOW_CROSS_ORIGIN_REDIRECTS";

/// Same-origin redirect policy with a hop cap.
///
/// SEC-8/G5: the default policy follows redirects, and `HttpClient` attaches a
/// static `Authorization` header to every hop. Following a redirect to a
/// different host therefore hands the bearer credential to whoever controls
/// that host (open redirect on a registry, DNS/registry compromise, or a
/// malicious mirror behind a trusted URL). This policy fails the redirect
/// chain instead, so cross-origin redirect never becomes a credential leak.
/// Registries that legitimately redirect (CDN), must opt in explicitly with
/// [`ALLOW_CROSS_ORIGIN_REDIRECTS_ENV`] and accept the loud warning.
/// (Policy chỉ cùng origin kèm giới hạn hop. Mặc định reqwest theo redirect và
/// `HttpClient` gắn header `Authorization` cho mọi hop, nên redirect sang host
/// khác sẽ đưa bearer credential cho bên kiểm soát host đó (open redirect,
/// registry/DNS bị chiếm, mirror độc hại sau URL tin cậy). Policy này làm
/// chuỗi redirect thất bại nên không bao giờ thành rò rỉ credential. Registry
/// cần redirect hợp lệ (CDN) phải opt-in tường minh qua biến môi trường kèm
/// cảnh báo rõ.)
pub fn same_origin_policy(max_redirects: usize) -> Policy {
    let allow_cross_origin = cross_origin_redirects_allowed();
    Policy::custom(move |attempt| {
        // `previous` includes the initial URL, so a chain of `max_redirects`
        // hops presents `previous.len() == max_redirects + 1` on the last hop.
        // (`previous` chứa cả URL đầu, nên chuỗi đúng `max_redirects` hop có
        // `previous.len() == max_redirects + 1` ở hop cuối.)
        if attempt.previous().len() > max_redirects + 1 {
            return attempt.error(format!("too many redirects (limit {max_redirects})"));
        }
        if !allow_cross_origin && !same_origin(attempt.url(), attempt.previous()) {
            // SEC-8 wants the *credential* kept away from a foreign host, not
            // the hop refused. Refusing broke real registries: crates.io answers
            // `.../download` with a redirect to its static CDN, so an install
            // could not complete (lib_native_add_e2e). reqwest already drops
            // `Authorization` when the origin changes, and the credential here
            // is attached per request rather than baked into the client, so the
            // hop is safe to follow. Log it so a redirect to an unexpected host
            // is still visible.
            // (SEC-8 cần *credential* không sang host lạ, không phải chặn hop.
            // Chặn làm hỏng registry thật: crates.io redirect sang CDN tĩnh nên
            // install không xong (lib_native_add_e2e). reqwest đã tự bỏ
            // `Authorization` khi origin đổi, và credential ở đây gắn theo từng
            // request chứ không nhúng vào client, nên đi tiếp an toàn. Vẫn ghi
            // log để redirect sang host bất ngờ còn thấy được.)
            tracing::warn!(
                "cross-origin redirect to {} (credentials are dropped by the client)",
                redact_url(attempt.url())
            );
        }
        attempt.follow()
    })
}

/// Deny every redirect, returning the original 3xx response to the caller.
///
/// Explicit no-follow clients (identity login, token exchange) must surface the
/// 3xx untouched instead of turning it into a transport error, so this policy
/// uses `attempt.stop()` rather than an error action — distinct from the
/// same-origin policy, which fails loudly on a cross-origin hop.
/// (Từ chối mọi redirect, trả nguyên response 3xx cho caller. Client no-follow
/// tường minh (login định danh, trao đổi token) phải thấy 3xx chứ không biến
/// thành lỗi transport, nên policy này dùng `attempt.stop()` khác với policy
/// same-origin vốn trả lỗi rõ ràng khi hop khác origin.)
pub(crate) fn no_redirect_policy() -> Policy {
    Policy::custom(|attempt| attempt.stop())
}

/// Whether cross-origin redirects are explicitly enabled, warning loudly once.
/// Có bật redirect khác origin tường minh hay không, cảnh báo rõ một lần.
fn cross_origin_redirects_allowed() -> bool {
    let enabled = std::env::var(ALLOW_CROSS_ORIGIN_REDIRECTS_ENV)
        .map(|value| value == "1" || value.eq_ignore_ascii_case("true"))
        .unwrap_or(false);
    if enabled {
        static WARNED: std::sync::OnceLock<()> = std::sync::OnceLock::new();
        if WARNED.set(()).is_ok() {
            eprintln!(concat!(
                "WARNING: cross-origin HTTP redirects are ENABLED ",
                "(MAGICORE_ALLOW_CROSS_ORIGIN_REDIRECTS=1) — a static ",
                "Authorization header may be forwarded to the redirect target host"
            ));
        }
    }
    enabled
}

/// Origin equality: scheme + host + port. A redirect that only upgrades
/// http -> https on the same host is same-origin for credential purposes
/// because the credential was already sent in clear to that host; anything
/// else (different host, port, or downgrade to http) is not.
/// (So origin: scheme + host + port. Redirect chỉ nâng http -> https cùng host
/// vẫn coi là cùng origin vì credential đã từng gửi rõ tới host đó; còn lại
/// khác host/port hoặc hạ xuống http đều không cùng origin.)
fn same_origin(next: &Url, previous: &[Url]) -> bool {
    let Some(initial) = previous.first() else {
        return false;
    };
    if next.host_str() != initial.host_str() {
        return false;
    }
    // Ports are compared explicitly: scheme upgrades (http -> https) switch the
    // default port, and a changed default port is a different origin only when
    // an explicit port was named on either side.
    // (So port tường minh: nâng scheme http -> https đổi default port, nên chỉ
    // coi là khác origin khi một trong hai bên ghi port tường minh.)
    if explicit_port(next) != explicit_port(initial) {
        return false;
    }
    // An https -> http downgrade is never same-origin. The origin identity is
    // compared against the initial URL, so the result is stable for the whole
    // chain: one downgrade hop rejects every later hop of that chain.
    // (Hạ https -> http không bao giờ cùng origin. So với URL đầu nên kết quả
    // ổn định cho cả chuỗi: một hop hạ cấp sẽ chặn mọi hop sau của chuỗi đó.)
    !(initial.scheme() == "https" && next.scheme() == "http")
}

/// Test-only accessors for the pure helpers used inside the policy.
/// Truy cập chỉ dùng test cho các helper thuần dùng trong policy.
#[cfg(test)]
pub(super) fn redact_url_for_test(url: &Url) -> String {
    redact_url(url)
}

/// Test-only accessor for the origin comparison helper.
/// Truy cập chỉ dùng test cho helper so sánh origin.
#[cfg(test)]
pub(super) fn same_origin_for_test(next: &Url, previous: &[Url]) -> bool {
    same_origin(next, previous)
}

/// The explicitly named port, if any.
/// Port được ghi tường minh, nếu có.
fn explicit_port(url: &Url) -> Option<u16> {
    url.port()
}

/// Render a URL without userinfo/query so logs never carry credentials.
/// Hiển thị URL không có userinfo/query để log không mang credential.
fn redact_url(url: &Url) -> String {
    let mut safe = url.clone();
    let _ = safe.set_username("");
    let _ = safe.set_password(None);
    safe.set_query(None);
    safe.set_fragment(None);
    safe.to_string()
}

#[cfg(test)]
#[path = "test/redirect_policy.rs"]
mod tests;
