//! Metadata policy.
// Các helper và chính sách metadata.

/// Accept header for packument fetches: abbreviated (small) normally,
/// FULL document when this client's age gate is armed (abbreviated docs
/// omit the `time` map the gate needs — fetching abbreviated under an
/// armed gate would silently keep everything).
/// (Header Accept: full doc khi cổng tuổi bật.)
pub(in crate::native::npm_registry) fn metadata_accept_header(armed: bool) -> &'static str {
    if armed {
        "application/json"
    } else {
        "application/vnd.npm.install-v1+json"
    }
}
