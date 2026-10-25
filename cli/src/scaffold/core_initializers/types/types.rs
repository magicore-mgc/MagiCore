//! Initializers for cores whose framework publishes a command-line scaffolder.
//! Initializer cho các core mà framework phát hành chương trình scaffold CLI.
//!
//! The Web core keeps its richer catalogue in `commands/core/web/create/
//! initializers/` because Web also runs registry packages and HTTP services.
//! This owner covers the remaining cores, where the framework's own binary is
//! the only source: Godot writes `project.godot`, PlatformIO writes
//! `platformio.ini`, CDK writes `cdk.json`, and so on. MagiCore runs that binary
//! and never supplies files of its own.
//! Core Web giữ catalog đầy đủ hơn ở owner riêng vì Web còn chạy registry
//! package và dịch vụ HTTP. Owner này phủ các core còn lại, nơi binary của
//! framework là nguồn duy nhất: Godot ghi `project.godot`, PlatformIO ghi
//! `platformio.ini`, CDK ghi `cdk.json`… MagiCore chạy binary đó và không tự
//! cung cấp file nào.

/// One command-line initializer for a non-Web core.
/// Một initializer CLI cho core không phải Web.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CoreInitializer {
    /// Ecosystem id the scaffold belongs to (`game`, `iot`, `clo`, `cicd`).
    /// Id ecosystem của scaffold.
    pub(crate) ecosystem: &'static str,
    /// Framework id inside that ecosystem.
    /// Id framework trong ecosystem đó.
    pub(crate) framework: &'static str,
    /// Program resolved through `PATH`.
    /// Chương trình resolve qua `PATH`.
    pub(crate) program: &'static str,
    /// Arguments before the project path.
    /// Tham số trước đường dẫn project.
    pub(crate) args_before: &'static [&'static str],
    /// Arguments after the project path.
    /// Tham số sau đường dẫn project.
    pub(crate) args_after: &'static [&'static str],
    /// Probe that proves the program is usable.
    /// Probe chứng minh chương trình dùng được.
    pub(crate) probe: &'static [&'static str],
    /// Install hint shown when the program is missing.
    /// Gợi ý cài hiện khi thiếu chương trình.
    pub(crate) install_hint: &'static str,
    /// Marker file proving the tool really produced this framework's project.
    /// File marker chứng minh tool thật sự sinh đúng project của framework.
    pub(crate) marker: &'static str,
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
