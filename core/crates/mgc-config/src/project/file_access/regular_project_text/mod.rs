//! Bounded no-follow text reader wiring.
//! Wiring bộ đọc văn bản có giới hạn, không theo symlink.
mod regular_project_text;
pub use regular_project_text::read_regular_project_text;
pub(super) use regular_project_text::read_regular_project_text_typed;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
