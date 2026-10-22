//! Represents evidence lost by a refused legacy-to-v4 conversion.
//! Biểu diễn bằng chứng bị mất khiến chuyển đổi legacy sang v4 bị từ chối.

/// Lossy migration refusal — the v3 rows cannot become v4 without dropping
/// evidence the installer needs (edges, peers, sources, SRI). The caller
/// keeps the v3 lock untouched and reports these warnings loudly.
/// (Từ chối migration hao hụt — row v3 không thành v4 được mà không rớt bằng
/// chứng installer cần. Caller giữ nguyên lock v3 và báo warnings rõ.)
#[derive(Debug, Clone)]
pub struct LossyMigration {
    /// Every dropped piece of evidence — Mọi bằng chứng bị rớt.
    pub warnings: Vec<String>,
}

impl std::fmt::Display for LossyMigration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "v3 lock cannot become v4 losslessly ({} gap(s)): {}; resolve or re-install to fill them, or stay on v3",
            self.warnings.len(),
            self.warnings.join("; ")
        )
    }
}

impl std::error::Error for LossyMigration {}
