// Bare core resolver — require a project core without an implicit Web fallback.
// Hàm này yêu cầu core dự án và không tự fallback sang Web.
use crate::dispatch::bare::error::BareRouteError;

pub(in crate::dispatch::bare) fn require_ecosystem<'a>(
    verb: &str,
    ecosystem: Option<&'a str>,
) -> anyhow::Result<&'a str> {
    ecosystem.ok_or_else(|| {
        BareRouteError::CoreNotDetected {
            verb: verb.to_string(),
        }
        .into_error()
    })
}
