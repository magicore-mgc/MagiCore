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
    /// How the tool receives the project path.
    /// Cách tool nhận đường dẫn project.
    pub(crate) destination: DestinationArg,
    /// Marker file proving the tool really produced this framework's project.
    /// File marker chứng minh tool thật sự sinh đúng project của framework.
    pub(crate) marker: &'static str,
    /// Where the tool takes a platform board or target that varies per project.
    ///
    /// A board baked into the catalog would be a version pinned in source: it
    /// goes stale, and it forces every project onto one piece of hardware. The
    /// caller supplies the value instead, and `None` means the framework has no
    /// such choice.
    /// Nơi tool nhận board hoặc target thay đổi theo project. Board ghi cứng
    /// trong catalog là một version ghim vào source: nó lỗi thời và ép mọi
    /// project dùng một phần cứng. Caller cung cấp giá trị thay thế, và `None`
    /// nghĩa là framework không có lựa chọn đó.
    pub(crate) board: Option<BoardFlag>,
}

/// How a scaffolder expects to be told where the project goes.
///
/// The distinction is not cosmetic. `cargo new --lib X` treats `X` as a project
/// NAME and rejects a path, while `mvn archetype:generate -DartifactId X` runs in
/// the directory and names the artifact. Feeding a path to the first form makes
/// the tool fail; feeding a name to the second scatters output into the caller's
/// working directory. Both mistakes looked like success in the previous catalog.
/// (Cách scaffolder nhận đường dẫn project. Khác biệt này không phải hình thức.
/// `cargo new --lib X` coi `X` là TÊN project và từ chối path, còn
/// `mvn archetype:generate -DartifactId X` chạy trong thư mục và đặt tên
/// artifact. Đưa path cho dạng đầu thì tool lỗi; đưa tên cho dạng sau thì output
/// rơi vào thư mục hiện tại. Cả hai lỗi từng trông như thành công trong catalog
/// cũ.)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DestinationArg {
    /// The path is a positional argument, e.g. `flutter create --template=app <path>`.
    /// Path là tham số vị trí.
    Positional,
    /// The path goes to a named flag, e.g. `pulumi new ... --dir <path>`.
    /// Path đi vào một flag có tên.
    Flag(&'static str),
    /// The tool runs INSIDE the destination and takes the project NAME as a
    /// positional argument, e.g. `go mod init <name>`.
    /// Tool chạy TRONG thư mục đích và nhận TÊN project làm tham số vị trí.
    NameInCwd,
}

/// How a board or target flag is passed to the tool.
/// Cách cờ board hoặc target được truyền cho tool.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct BoardFlag {
    /// Flag the tool expects, e.g. `--board`.
    /// Cờ tool mong đợi, ví dụ `--board`.
    pub(crate) flag: &'static str,
    /// Value used when the project does not choose one.
    ///
    /// A default belongs here rather than in the argument list: it documents
    /// that the value is a default, not a fixed truth.
    /// Giá trị dùng khi project không chọn. Mặc định thuộc chỗ này thay vì
    /// trong danh sách tham số: nó ghi rõ đây là mặc định, không phải sự thật
    /// cố định.
    pub(crate) default: &'static str,
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
