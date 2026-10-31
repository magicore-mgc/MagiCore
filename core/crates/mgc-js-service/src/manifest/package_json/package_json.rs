//! Native package.json schema and safe read/write entrypoints.
//! Schema package.json native và điểm vào đọc/ghi an toàn.

use crate::manifest::atomic_write::atomic_write_if_changed;
use crate::manifest::error::ManifestError;
use mgc_platform::OpenRegularFileError;
use mgc_types::{MgError, MgResult};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::io::Read;
use std::path::Path;

pub(in crate::manifest) const MAX_MANIFEST_SIZE: u64 = 10 * 1024 * 1024;

/// Preserve common npm fields and unknown JSON properties.
/// Giữ các field npm phổ biến và thuộc tính JSON chưa biết.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageJson {
    pub name: String,
    // Empty means the local application omitted its optional version field.
    // Chuỗi rỗng biểu diễn field version vắng mặt và không được ghi ngược ra JSON.
    #[serde(
        default,
        deserialize_with = "deserialize_version",
        skip_serializing_if = "String::is_empty"
    )]
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "devDependencies")]
    pub dev_dependencies: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "peerDependencies")]
    pub peer_dependencies: Option<std::collections::HashMap<String, String>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "optionalDependencies"
    )]
    pub optional_dependencies: Option<std::collections::HashMap<String, String>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

fn deserialize_version<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let version = String::deserialize(deserializer)?;
    if version.is_empty() {
        return Err(serde::de::Error::custom(
            "package version must not be empty",
        ));
    }
    Ok(version)
}

impl PackageJson {
    /// Construct a minimal package manifest.
    /// Tạo manifest package tối thiểu.
    pub fn new(name: String, version: String) -> Self {
        Self {
            name,
            version,
            description: None,
            dependencies: None,
            dev_dependencies: None,
            peer_dependencies: None,
            optional_dependencies: None,
            extra: Map::new(),
        }
    }

    /// Load package.json without following a final symlink.
    /// Đọc package.json mà không đi theo symlink cuối.
    pub fn load(path: &Path) -> Result<Self, anyhow::Error> {
        Ok(load_checked(path)?)
    }

    /// Serialize and atomically write only when bytes changed.
    ///
    /// The document always ends with exactly one `\n`: npm, editors and
    /// `git diff` treat a missing final newline as a "\ No newline at end of
    /// file" change, and `mgc update` used to strip the newline a project
    /// already had. Trailing blank lines collapse to that single newline so
    /// repeated writes stay byte-stable.
    /// (Luôn kết thúc đúng một `\n`: npm/editor/git diff coi thiếu newline
    /// cuối file là thay đổi "\ No newline at end of file", và `mgc update`
    /// từng xoá newline có sẵn. Dòng trống cuối gộp về đúng một newline để
    /// ghi lặp lại giữ nguyên byte.)
    pub fn save(&self, path: &Path) -> Result<(), anyhow::Error> {
        let content = serialize_package_json(self)?;
        atomic_write_if_changed(path, content.as_bytes())?;
        Ok(())
    }
}

/// Pretty JSON with exactly one trailing newline — JSON đẹp với đúng một
/// newline cuối.
pub(in crate::manifest) fn serialize_package_json(
    package: &PackageJson,
) -> Result<String, serde_json::Error> {
    let mut content = serde_json::to_string_pretty(package)?;
    let trimmed_len = content.trim_end_matches(['\n', '\r']).len();
    content.truncate(trimmed_len);
    content.push('\n');
    Ok(content)
}

/// Open once, enforce the size limit on that handle, and deserialize JSON.
/// Mở một lần, chặn kích thước trên đúng handle rồi deserialize JSON.
pub(in crate::manifest) fn load_checked(path: &Path) -> MgResult<PackageJson> {
    let file = match mgc_platform::open_regular_file_no_follow(path) {
        Ok(file) => file,
        Err(OpenRegularFileError::Io(source)) => return Err(MgError::Io(source)),
        Err(source) => return Err(ManifestError::OpenPackageJson { source }.into()),
    };
    let metadata = file.metadata()?;
    if metadata.len() > MAX_MANIFEST_SIZE {
        return Err(ManifestError::ManifestTooLarge {
            size: metadata.len(),
            max: MAX_MANIFEST_SIZE,
        }
        .into());
    }

    let mut content = String::new();
    let mut bounded_file = file.take(MAX_MANIFEST_SIZE + 1);
    bounded_file.read_to_string(&mut content)?;
    if content.len() as u64 > MAX_MANIFEST_SIZE {
        return Err(ManifestError::ManifestTooLarge {
            size: content.len() as u64,
            max: MAX_MANIFEST_SIZE,
        }
        .into());
    }
    Ok(serde_json::from_str(&content)?)
}
