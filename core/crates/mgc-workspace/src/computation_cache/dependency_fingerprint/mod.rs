// dependency_fingerprint owner facade — mặt nối owner dependency_fingerprint

mod dependency_fingerprint;
pub use dependency_fingerprint::compute_composite_hash;
#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
