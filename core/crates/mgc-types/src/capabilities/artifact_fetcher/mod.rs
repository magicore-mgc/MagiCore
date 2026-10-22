//! Artifact fetch capability module wiring.
//! Wiring module capability tải artifact.

mod artifact_fetcher;
pub use self::artifact_fetcher::ArtifactFetcher;
#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
