//! Project-manifest probes shared by the command families.
//! Probe manifest project dùng chung giữa các họ lệnh.
//!
//! Each command family used to carry its own copy of "does this root hold a
//! `.csproj`?", which is how `mgc test` and the optimizer ended up recognising
//! different sets of languages. One owner means one answer.
//! (Mỗi họ lệnh từng tự giữ bản sao "root này có `.csproj` không?", và đó là
//! cách `mgc test` với optimizer nhận diện hai tập ngôn ngữ khác nhau.)

use std::io;
use std::path::Path;

/// Whether `project_root` holds a regular file whose extension matches.
///
/// .NET project files are named after the project (`App.csproj`), so a fixed
/// filename lookup cannot find them. The scan is non-recursive, ignores case
/// (Windows tooling writes both), and requires a regular file so a directory
/// named `Trap.csproj` cannot be mistaken for a project.
/// (File project .NET đặt tên theo project nên không tra tên cố định được. Quét
/// không đệ quy, không phân biệt hoa thường, và phải là regular file để thư mục
/// tên `Trap.csproj` không bị nhầm.)
pub(crate) fn has_file_with_extension(project_root: &Path, extension: &str) -> io::Result<bool> {
    let entries = match std::fs::read_dir(project_root) {
        Ok(entries) => entries,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(false),
        Err(error) => return Err(error),
    };
    for entry in entries {
        let entry = entry?;
        if !entry
            .path()
            .extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|ext| ext.eq_ignore_ascii_case(extension))
        {
            continue;
        }
        if entry.file_type()?.is_file() {
            return Ok(true);
        }
    }
    Ok(false)
}

/// Whether the project declares a .NET project or solution file.
/// Project có file project/solution .NET hay không.
pub fn declares_dotnet_project(project_root: &Path) -> bool {
    has_file_with_extension(project_root, "csproj").unwrap_or(false)
        || has_file_with_extension(project_root, "sln").unwrap_or(false)
}
