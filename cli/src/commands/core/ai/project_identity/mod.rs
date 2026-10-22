//! AI project-identity owner wiring.
//! Wiring owner nhận diện project AI.

mod project_identity;

#[cfg(feature = "ai")]
pub use project_identity::ai_project_root;
