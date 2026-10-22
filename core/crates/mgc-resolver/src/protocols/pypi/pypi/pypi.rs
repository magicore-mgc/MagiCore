//! Native PyPI protocol facade and coordinator.
//! Facade và coordinator protocol PyPI native.

use crate::protocols::pypi::PypiAgeGate;

const DEFAULT_INDEX_URL: &str = "https://pypi.org";

/// Native PyPI engine.
/// Engine native PyPI.
#[derive(Debug, Clone)]
pub struct PypiProtocol {
    pub(in crate::protocols::pypi) index_url: String,
    pub(in crate::protocols::pypi) client: mgc_http::HttpClient,
    pub(in crate::protocols::pypi) age_gate: Option<PypiAgeGate>,
}

impl PypiProtocol {
    /// Build with an explicit index URL.
    /// Dựng với index URL tường minh.
    pub fn new(index_url: &str) -> Self {
        Self {
            index_url: index_url.trim_end_matches('/').to_string(),
            client: mgc_http::HttpClient::default(),
            age_gate: None,
        }
    }

    /// Arm the per-operation PyPI release-age gate; zero hours disables it.
    /// Bật cổng tuổi phát hành PyPI cho operation; 0 giờ sẽ tắt cổng.
    pub fn with_age_gate(mut self, cutoff_hours: u64, allow_missing_time: bool) -> Self {
        self.age_gate = (cutoff_hours > 0).then_some(PypiAgeGate {
            cutoff_hours,
            allow_missing_time,
        });
        self
    }

    /// Build from environment: `MGC_PYPI_INDEX_URL`.
    /// Dựng từ môi trường: `MGC_PYPI_INDEX_URL`.
    pub fn from_env() -> Self {
        let index_url = std::env::var("MGC_PYPI_INDEX_URL")
            .ok()
            .filter(|v| !v.is_empty())
            .unwrap_or_else(|| DEFAULT_INDEX_URL.to_string());
        Self::new(&index_url)
    }
}

impl Default for PypiProtocol {
    fn default() -> Self {
        Self::from_env()
    }
}

#[cfg(test)]
#[path = "test/pypi.rs"]
mod tests;

#[cfg(test)]
#[path = "test/importable.rs"]
mod importable_tests;

#[cfg(test)]
#[path = "test/markers.rs"]
mod marker_tests;
