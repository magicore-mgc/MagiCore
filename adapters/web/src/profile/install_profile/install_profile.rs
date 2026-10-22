//! Install profiling lifecycle and timing marks.
//! Vòng đời và dấu thời gian profiling install.

use std::time::Instant;

#[derive(Default)]
pub struct InstallProfile {
    pub enabled: bool,
    pub marks: Vec<(&'static str, u128)>,
}

impl InstallProfile {
    pub fn from_env() -> Self {
        let enabled = std::env::var("MAGICORE_WEB_PROFILE_INSTALL")
            .ok()
            .map(|value| value.trim().to_ascii_lowercase())
            .map(|value| matches!(value.as_str(), "1" | "true" | "yes" | "on"))
            .unwrap_or(false);
        Self {
            enabled,
            marks: Vec::new(),
        }
    }

    pub fn mark(&mut self, label: &'static str, started_at: Instant) {
        if self.enabled {
            self.marks.push((label, started_at.elapsed().as_millis()));
        }
    }

    pub fn mark_step(&mut self, label: &'static str, step_started_at: Instant) {
        if self.enabled {
            self.marks
                .push((label, step_started_at.elapsed().as_millis()));
        }
    }

    pub fn flush(&self, total_ms: u64) {
        if !self.enabled {
            return;
        }

        eprintln!("[magicore:web:install-profile] total={}ms", total_ms);
        for (label, millis) in &self.marks {
            eprintln!("[magicore:web:install-profile] {}={}ms", label, millis);
        }
    }
}

#[cfg(test)]
#[path = "test/install_profile.rs"]
mod tests;
