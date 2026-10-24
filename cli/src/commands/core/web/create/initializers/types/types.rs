//! Data contract for one framework initializer.
//! Contract dữ liệu cho một initializer của framework.
//!
//! An initializer record names *who publishes* the scaffolding program and
//! *how to invoke it*. It never carries framework source: the framework team
//! owns the files, MagiCore owns the invocation, the provenance and the project
//! metadata. That separation is what keeps this repository free of embedded
//! framework code.
//! Record initializer nêu *ai phát hành* chương trình scaffold và *cách gọi*.
//! Nó không bao giờ mang source framework: framework team sở hữu file,
//! MagiCore sở hữu lời gọi, provenance và metadata project. Sự tách bạch đó
//! giữ repo này không nhúng code framework.

/// One argument position in the initializer invocation.
/// Một vị trí tham số trong lời gọi initializer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ArgSlot {
    /// Replaced by the user-supplied project name.
    /// Được thay bằng tên project do user cung cấp.
    Project,
    /// Passed verbatim to the initializer binary.
    /// Truyền nguyên văn cho binary initializer.
    Literal(&'static str),
}

/// Where the initializer writes the generated project.
/// Initializer ghi project đã sinh ở đâu.
///
/// `InPlace` has no record yet: every current initializer takes a project name
/// and writes a subdirectory. The variant exists so adding an in-place
/// initializer does not require reshaping the enum, and the catalog test pins
/// the current state.
/// `InPlace` chưa có record nào: mọi initializer hiện tại đều nhận tên project
/// và ghi thư mục con. Variant tồn tại để thêm initializer in-place không phải
/// đổi shape enum, và test catalog ghim trạng thái hiện tại.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub(crate) enum Layout {
    /// `<cwd>/<project>/` — the initializer takes the project name as an argument.
    /// `<cwd>/<project>/` — initializer nhận tên project làm tham số.
    NestedProject,
    /// `<cwd>/` — the initializer fills the working directory itself.
    /// `<cwd>/` — initializer ghi đầy vào thư mục làm việc.
    InPlace,
}

/// How tightly the initializer package version is pinned.
/// Mức ghim phiên bản package initializer.
///
/// Every record currently follows the registry dist-tag. `Pinned` and
/// `Compatible` exist so a framework that breaks its `latest` starter can be
/// held to a known-good release without changing the data contract.
/// Mọi record hiện theo dist-tag. `Pinned` và `Compatible` tồn tại để framework
/// làm hỏng starter `latest` có thể bị giữ ở bản đã biết tốt mà không phải đổi
/// contract dữ liệu.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub(crate) enum VersionPolicy {
    /// Follow the registry dist-tag.
    /// Theo dist-tag của registry.
    Latest,
    /// Exactly this version.
    /// Đúng phiên bản này.
    Pinned(&'static str),
    /// A caret range around a known-good release.
    /// Range caret quanh bản đã biết tốt.
    Compatible(&'static str),
}

impl VersionPolicy {
    /// Registry selector this policy asks the package runtime for.
    /// Selector registry mà policy này yêu cầu package runtime dùng.
    pub(crate) fn selector(self) -> &'static str {
        match self {
            VersionPolicy::Latest => "latest",
            VersionPolicy::Pinned(version) => version,
            VersionPolicy::Compatible(version) => version,
        }
    }
}

/// How the initializer produces a project.
/// Cách initializer sinh ra project.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Runtime {
    /// A program published to a package registry, run through the package
    /// runtime (`create-vite`, `create-next-app`, `sv`, …).
    /// Chương trình phát hành lên registry, chạy qua package runtime.
    RegistryPackage,
    /// A program the user already has on `PATH`, run directly
    /// (`django-admin`, `composer`, `rails`). MagiCore checks the program
    /// answers `--version` before using it, so a missing or broken toolchain is
    /// reported instead of producing half a project.
    /// Chương trình user đã có trên `PATH`, chạy trực tiếp. MagiCore kiểm
    /// chương trình trả lời `--version` trước khi dùng, nên toolchain thiếu
    /// hoặc hỏng được báo thay vì sinh nửa project.
    HostTool {
        /// Program name resolved through `PATH`.
        /// Tên chương trình resolve qua `PATH`.
        program: &'static str,
        /// Message shown when the program is missing, with the install hint.
        /// Thông điệp khi thiếu chương trình, kèm gợi ý cài.
        install_hint: &'static str,
        /// Subcommand chain for the version probe, e.g. `["--version"]`.
        /// Chuỗi subcommand cho probe phiên bản.
        probe: &'static [&'static str],
    },
    /// An archive served by the framework's own project generator service.
    /// Archive do dịch vụ sinh project của chính framework phục vụ.
    HttpArchive {
        /// Base URL of the generator service.
        /// URL gốc của dịch vụ sinh project.
        endpoint: &'static str,
        /// Query parameters appended verbatim, in order.
        /// Tham số query nối nguyên văn, theo thứ tự.
        params: &'static [(&'static str, &'static str)],
        /// Query parameter that carries the project name.
        /// Tham số query mang tên project.
        name_param: &'static str,
        /// Set when the archive wraps its project in a single top-level
        /// directory that must be hoisted, as `code.quarkus.io` does.
        /// Đặt khi archive bọc project trong một thư mục con duy nhất cần
        /// hoist lên, như `code.quarkus.io` làm.
        unwrap_single_root: bool,
    },
}

/// One official framework initializer.
/// Một initializer chính thức của framework.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct InitializerRecord {
    /// Registry package that publishes the scaffolding program.
    /// Package registry phát hành chương trình scaffold.
    pub(crate) package: &'static str,
    /// Binary inside that package which performs scaffolding.
    /// Binary bên trong package thực hiện scaffold.
    pub(crate) binary: &'static str,
    /// Stable identity written into the scaffold provenance.
    /// Identity ổn định ghi vào provenance của scaffold.
    pub(crate) identity: &'static str,
    /// Argument template; `Project` slots receive the project name.
    /// Mẫu tham số; vị trí `Project` nhận tên project.
    pub(crate) args: &'static [ArgSlot],
    /// Where the generated project appears.
    /// Project đã sinh xuất hiện ở đâu.
    pub(crate) layout: Layout,
    /// Package version policy.
    /// Chính sách phiên bản package.
    pub(crate) version: VersionPolicy,
    /// How the project is produced.
    /// Cách project được sinh ra.
    pub(crate) runtime: Runtime,
}

/// Scaffold mode a record belongs to.
/// Mode scaffold mà record thuộc về.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Mode {
    Frontend,
    Backend,
}

impl Mode {
    /// Parse a scaffold sub-type into a mode, rejecting anything else.
    /// Parse sub-type scaffold thành mode, từ chối giá trị khác.
    pub(crate) fn parse(sub_type: &str) -> Option<Self> {
        match sub_type {
            "frontend" | "fullstack" => Some(Mode::Frontend),
            "backend" => Some(Mode::Backend),
            _ => None,
        }
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
