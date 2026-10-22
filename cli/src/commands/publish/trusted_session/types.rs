#[derive(Clone)]
pub(in crate::commands) struct TrustedPublishSession {
    pub(in crate::commands::publish) token: String,
    pub(in crate::commands::publish) expires_at: tokio::time::Instant,
    pub(in crate::commands::publish) registry: String,
    pub(in crate::commands::publish) package: String,
    pub(in crate::commands::publish) protocol: String,
    pub(in crate::commands::publish) audience: Option<String>,
}
