//! Adds stable request headers and header-only bearer authentication.
//! Thêm header ổn định và xác thực bearer chỉ qua header.

/// Security headers & token handling (12 §10)
/// Quản lý security header và token.
pub fn secure_request_builder(
    builder: reqwest::RequestBuilder,
    token: Option<&str>,
) -> reqwest::RequestBuilder {
    let mut b = builder
        .header(
            "User-Agent",
            format!("magicore/{} (os; arch)", env!("CARGO_PKG_VERSION")),
        )
        .header("Accept", "application/json");

    if let Some(t) = token {
        // Token is only sent in the Authorization header, never in the query string.
        // Token chỉ trong header Authorization - KHÔNG query string.
        b = b.bearer_auth(t);
    }
    b
}

#[cfg(test)]
#[path = "test/secure_request_builder.rs"]
mod tests;
