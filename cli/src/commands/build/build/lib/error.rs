//! Typed failures owned by native library build routes.
//! Lỗi typed thuộc các route build native của thư viện.

use std::fmt;
use std::path::PathBuf;

/// Marker the shared remedy appends when it names the install command.
///
/// A single named marker beats substring-matching an advice sentence: the
/// display only suppresses its generic note when a remedy really was attached.
/// (Marker do phần khắc phục dùng chung thêm vào khi nêu lệnh cài. Một marker có
/// tên tốt hơn match chuỗi câu khuyên: Display chỉ bỏ câu chung khi thật sự có
/// kèm khắc phục.)
const PYTHON_REMEDY_MARKER: &str = "pip install";

/// Compiler and unsupported-language failures specific to the Lib build lane.
#[derive(Debug)]
pub(super) enum LibBuildError {
    ToolchainMissing { tool: String },
    PythonBackendFailed { source: anyhow::Error },
    GoBuildFailed { source: anyhow::Error },
    DotnetBuildFailed { source: anyhow::Error },
    JavaBackendUnavailable { root: PathBuf },
}

impl fmt::Display for LibBuildError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ToolchainMissing { tool } => write!(
                formatter,
                "'{tool}' not found in PATH — install it first (mgc doctor check lists required toolchains)"
            ),
            // A missing backend arrives with the exact install command
            // attached, so the generic "provide a backend" note would repeat
            // the same advice; other backend failures keep it.
            // (Backend thiếu đã kèm đúng lệnh cài nên câu chung "hãy cung cấp
            // backend" sẽ lặp lại lời khuyên; lỗi backend khác giữ câu đó.)
            Self::PythonBackendFailed { source } => {
                let detail = source.to_string();
                write!(formatter, "Python build backend failed: {detail}")?;
                if !detail.contains(PYTHON_REMEDY_MARKER) {
                    write!(
                        formatter,
                        " — MagiCore does not install Python build tools; provide a supported build backend before retrying"
                    )?;
                }
                Ok(())
            }
            Self::GoBuildFailed { source } => write!(formatter, "go build ./... failed: {source}"),
            Self::DotnetBuildFailed { source } => {
                write!(
                    formatter,
                    "dotnet build failed: {source} — install the .NET SDK first"
                )
            }
            Self::JavaBackendUnavailable { root } => write!(
                formatter,
                "Java build descriptor found in '{}', but MagiCore has no native Java build backend yet; refusing to route the project through TypeScript or invoke Maven/Gradle",
                root.display()
            ),
        }
    }
}

impl std::error::Error for LibBuildError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::PythonBackendFailed { source }
            | Self::GoBuildFailed { source }
            | Self::DotnetBuildFailed { source } => Some(source.as_ref()),
            Self::ToolchainMissing { .. } | Self::JavaBackendUnavailable { .. } => None,
        }
    }
}
