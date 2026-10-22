use std::time::Instant;

#[derive(Default)]
pub(in crate::solver::resolver) struct ResolverProfile {
    enabled: bool,
}

impl ResolverProfile {
    pub(in crate::solver::resolver) fn from_env() -> Self {
        let enabled = std::env::var("MAGICORE_RESOLVER_PROFILE")
            .ok()
            .map(|value| value.trim().to_ascii_lowercase())
            .map(|value| matches!(value.as_str(), "1" | "true" | "yes" | "on"))
            .unwrap_or(false);
        Self { enabled }
    }

    pub(in crate::solver::resolver) fn mark(&self, label: &str, started_at: Instant) {
        if self.enabled {
            eprintln!(
                "[magicore:resolver-profile] {}={}ms",
                label,
                started_at.elapsed().as_millis()
            );
        }
    }
}
