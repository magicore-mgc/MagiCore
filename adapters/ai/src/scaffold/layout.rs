//! Destination layout shared by every AI framework generator.
//! Bố cục đích dùng chung cho generator của mọi framework AI.
//!
//! Generators write only into a tree the caller owns. A create never merges
//! into an existing project, and a symlink at a destination is refused so the
//! writer cannot be redirected outside that tree.
//! Generator chỉ ghi vào cây do caller sở hữu. Lệnh create không hòa vào
//! project đã có, và symlink tại đích bị từ chối để writer không bị chuyển
//! hướng ra ngoài cây.

use std::path::{Path, PathBuf};

use mgc_types::MgResult;

use super::slug::{distribution_name, module_name};
use crate::framework::AiFramework;

/// Minimum Python interpreter version declared by generated projects.
/// Phiên bản Python tối thiểu mà project sinh ra khai báo.
pub(super) const MIN_PYTHON_VERSION: &str = "3.10";

/// Default runtime dependency set for an agent-style project.
/// Bộ dependency runtime mặc định cho project dạng agent.
pub(super) const AGENT_DEPENDENCIES: &[&str] = &["openai", "httpx", "pydantic", "python-dotenv"];

/// Default runtime dependency set for an MCP stdio server.
/// Bộ dependency runtime mặc định cho server MCP stdio.
pub(super) const MCP_DEPENDENCIES: &[&str] = &["pydantic", "python-dotenv"];

/// Paths one AI project skeleton writes to.
/// Các đường dẫn mà một skeleton project AI ghi tới.
pub(super) struct ProjectLayout {
    /// Directory files are written into (the caller's tree).
    /// Thư mục ghi file (cây của caller).
    root: PathBuf,
    /// Python package directory (`src/<package>/`).
    /// Thư mục package Python (`src/<package>/`).
    package_dir: PathBuf,
    /// Importable Python module name — tên module Python import được.
    package: String,
    /// Distribution name used by the packaging manifest — tên distribution.
    distribution: String,
}

impl ProjectLayout {
    /// Derive the layout for `project_name` under `root`.
    /// Suy bố cục cho `project_name` dưới `root`.
    pub(super) fn new(root: &Path, project_name: &str) -> Self {
        let package = module_name(project_name);
        Self {
            package_dir: root.join("src").join(&package),
            distribution: distribution_name(project_name),
            package,
            root: root.to_path_buf(),
        }
    }

    /// Importable module name — tên module import được.
    pub(super) fn package(&self) -> &str {
        &self.package
    }

    /// Packaging distribution name — tên distribution đóng gói.
    pub(super) fn distribution(&self) -> &str {
        &self.distribution
    }

    /// Write one regular file relative to the project root.
    /// Ghi một file thường tương đối so với gốc project.
    pub(super) fn write(&self, relative: &str, contents: &str) -> MgResult<()> {
        write_regular_file(&self.root.join(relative), contents)
    }

    /// Write one file inside the Python package.
    /// Ghi một file trong package Python.
    pub(super) fn write_package_file(&self, relative: &str, contents: &str) -> MgResult<()> {
        write_regular_file(&self.package_dir.join(relative), contents)
    }
}

/// Render `pyproject.toml` for `engine`, declaring the framework marker.
/// Kết xuất `pyproject.toml` cho `engine`, khai báo marker framework.
///
/// `[tool.magicore].framework` is read by the adapter's framework detection and
/// the PyPI ownership predicate; `[project].dependencies` stays inside the
/// simple `name` / `name>=version` subset that predicate can model, so the
/// generated project is manageable by MagiCore from the first command.
/// `[tool.magicore].framework` được adapter và predicate sở hữu PyPI đọc;
/// `[project].dependencies` chỉ nằm trong tập `name` / `name>=version` đơn
/// giản mà predicate mô hình được, nên project sinh ra quản lý được ngay.
pub(super) fn pyproject_toml(layout: &ProjectLayout, engine: AiFramework) -> String {
    let dependencies = match engine {
        AiFramework::PythonAgent => AGENT_DEPENDENCIES,
        AiFramework::McpServer => MCP_DEPENDENCIES,
    };
    let mut rendered = String::new();
    rendered.push_str("[build-system]\n");
    rendered.push_str("requires = [\"setuptools>=68\"]\n");
    rendered.push_str("build-backend = \"setuptools.build_meta\"\n\n");
    rendered.push_str("[project]\n");
    rendered.push_str(&format!("name = \"{}\"\n", layout.distribution()));
    rendered.push_str("version = \"0.1.0\"\n");
    rendered.push_str(&format!(
        "description = \"{description}\"\n",
        description = engine.description()
    ));
    rendered.push_str(&format!("requires-python = \">={MIN_PYTHON_VERSION}\"\n"));
    rendered.push_str("dependencies = [\n");
    for dependency in dependencies {
        rendered.push_str(&format!("  \"{dependency}\",\n"));
    }
    rendered.push_str("]\n\n");
    rendered.push_str("[project.scripts]\n");
    rendered.push_str(&format!(
        "{name} = \"{package}.__main__:main\"\n\n",
        name = layout.distribution(),
        package = layout.package()
    ));
    rendered.push_str("[tool.setuptools]\n");
    rendered.push_str("package-dir = { \"\" = \"src\" }\n\n");
    rendered.push_str("[tool.setuptools.packages.find]\n");
    rendered.push_str("where = [\"src\"]\n\n");
    rendered.push_str("[tool.magicore]\n");
    rendered.push_str(&format!("framework = \"{}\"\n", engine.as_str()));
    rendered
}

/// Write one regular file, creating parents and refusing a symlink destination.
/// Ghi một file thường, tạo thư mục cha và từ chối đích là symlink.
fn write_regular_file(path: &Path, contents: &str) -> MgResult<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    // A pre-existing symlink would redirect the write outside the project tree.
    // (Symlink sẵn có sẽ chuyển hướng ghi ra ngoài cây project.)
    if std::fs::symlink_metadata(path)
        .map(|metadata| metadata.file_type().is_symlink())
        .unwrap_or(false)
    {
        return Err(refused_destination(path).into());
    }
    std::fs::write(path, contents)?;
    Ok(())
}

/// Error for a destination that is a symlink instead of a regular file.
/// Lỗi cho đích là symlink thay vì file thường.
fn refused_destination(path: &Path) -> anyhow::Error {
    anyhow::anyhow!(
        "refusing to write scaffold file '{}': destination is a symlink",
        path.display()
    )
}
