//! Project Bun/Deno lock discovery — khám phá lock Bun/Deno trong dự án.
//! Reads each supported JS-runtime lockfile through the shared bounded reader.
//! Đọc mỗi lockfile runtime JS được hỗ trợ qua bounded reader dùng chung.

mod project_lockfiles;

pub use project_lockfiles::read_js_lockfiles;

#[cfg(test)]
#[path = "test/project_lockfiles.rs"]
mod tests;
