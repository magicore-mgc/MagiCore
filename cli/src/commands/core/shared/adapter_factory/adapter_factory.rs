use super::*;

/// Adapter của core — wrapper factory (expect giữ fail-closed như từng core cũ).
#[cfg(any(feature = "game", feature = "clo"))]
pub fn core_adapter(eco: &Ecosystem) -> Arc<dyn PackageAdapter> {
    crate::factory::create_adapter(eco, None, None)
        .expect("core adapter always available in this core build")
}
