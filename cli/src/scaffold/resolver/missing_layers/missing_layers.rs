//! Aggregates layer misses and formats one actionable diagnostic.
//! Gom layer bị thiếu và định dạng một thông báo có hướng xử lý.

use std::fmt;

/// Aggregated missing layers report (gom nhiều lỗi thành một)
#[derive(Debug, Default)]
pub struct MissingLayersReport {
    pub required: Vec<String>,
    pub optional: Vec<String>,
}

impl MissingLayersReport {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_required(&mut self, layer: String) {
        self.required.push(layer);
    }

    /// Record a layer resolution failure without discarding its cause.
    /// (Giữ nguyên nguyên nhân thật khi resolve layer thất bại.)
    pub fn add_required_failure(&mut self, layer: &str, error: impl fmt::Display) {
        self.required
            .push(format!("{layer} (resolution failed: {error})"));
    }

    pub fn add_optional(&mut self, layer: String) {
        self.optional.push(layer);
    }

    pub fn has_required_missing(&self) -> bool {
        !self.required.is_empty()
    }

    pub fn is_empty(&self) -> bool {
        self.required.is_empty() && self.optional.is_empty()
    }

    /// Format thành error message duy nhất thay vì spam warnings
    pub fn format_error(&self, core: &str, template: &str) -> String {
        let mut msg = String::new();

        if !self.required.is_empty() {
            msg.push_str(&format!(
                "Required scaffold layers missing ({} total):\n",
                self.required.len()
            ));
            for layer in &self.required {
                msg.push_str(&format!("  - {}\n", layer));
            }
            msg.push_str(&format!(
                "\nRun: mgc template fetch {} {}@latest\n",
                core, template
            ));
        }

        if !self.optional.is_empty() {
            if !msg.is_empty() {
                msg.push('\n');
            }
            msg.push_str(&format!(
                "Optional layers not found ({} total) - scaffold may have reduced features\n",
                self.optional.len()
            ));
        }

        msg
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
