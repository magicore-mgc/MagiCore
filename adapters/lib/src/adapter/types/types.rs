use crate::language::LibLanguage;
use std::path::PathBuf;

/// Which build manifest owns a Java lib project — Maven pom.xml is natively
/// parseable/resolvable; Gradle build scripts are toolchain programs and can
/// only fail closed honestly.
/// Build manifest nào sở hữu project lib Java — pom.xml của Maven parse và
/// resolve native được; build script Gradle là chương trình toolchain, chỉ
/// có thể fail-closed trung thực.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(in crate::adapter) enum JavaManifestKind {
    Pom,
    Gradle,
    Ambiguous,
    #[default]
    None,
}
pub struct LibAdapter {
    pub(in crate::adapter) language: LibLanguage,
    /// Security policy scope; the AI core embeds this adapter only for its
    /// native Python lane and must retain AI overrides from mgc.toml.
    /// Phạm vi policy bảo mật; engine Python nhúng của AI phải giữ override AI.
    pub(in crate::adapter) security_core: &'static str,
    /// Java-only: which build manifest the project carries (detected at
    /// construction, when the project root is known).
    /// (Chỉ cho Java: project mang build manifest nào (detect lúc dựng, khi
    /// đã biết project root).)
    pub(in crate::adapter) java_kind: JavaManifestKind,
    /// Project root is retained so Python ownership can be revalidated at
    /// each operation; foreign lockfiles may appear after construction.
    /// (Giữ project root để kiểm tra lại quyền sở hữu Python mỗi thao tác.)
    pub(in crate::adapter) project_root: PathBuf,
    /// .NET-only: the project's `<TargetFramework>` (first of
    /// `<TargetFrameworks>`), read at construction for multi-TFM group
    /// selection during native resolve. `None` = unknown → divergent
    /// multi-TFM sets fail closed.
    /// (Chỉ cho .NET: `<TargetFramework>` của project, đọc lúc dựng.)
    pub(in crate::adapter) dotnet_tfm: Option<String>,
    pub(in crate::adapter) web: Option<mgc_web_adapter::WebAdapter>,
    /// Lock v3 entries produced by the native resolve — flushed to
    /// mgc.lock during install.
    /// (Entry lock v3 sinh từ resolve native — ghi xuống mgc.lock lúc
    /// install.)
    pub(in crate::adapter) pending_lock: std::sync::Mutex<Vec<mgc_lockfile::Package>>,
}
/// Native resolve-first lane for one lib language — the protocol engine
/// that owns version selection (no toolchain spawn). `None` is impossible
/// here (TS rides the embedded web delegate, not this enum).
/// (Lane resolve-first native theo ngôn ngữ lib.)
pub(in crate::adapter) enum ResolveFirst {
    Python,
    Rust,
    Go,
    DotNet,
    JavaPom,
}
