//! Python native-lane ownership predicates.
//! Predicate xác định project và dependency group MGC được phép quản lý.

use std::path::Path;

use crate::manifest::dependency::parse_python_dependency;

/// Whether MGC can own Python runtime requirements and the supported PEP 735
/// `dev` group without ignoring another manager's lock or dependency source.
/// PEP 621 optional extras remain inactive unless explicitly selected.
/// (Chỉ nhận lane Python native khi MGC quản lý runtime + nhóm `dev` mà
/// không bỏ qua lockfile/nguồn dependency do tool khác quản lý.)
pub fn supports_native_python_project(root: &Path) -> bool {
    let manifest_path = root.join("pyproject.toml");
    let Ok(content) =
        mgc_adapter_base::project_file::read_regular_text(&manifest_path, "pyproject.toml")
    else {
        return false;
    };
    let Ok(document) = toml::from_str::<toml::Value>(&content) else {
        return false;
    };
    let Some(project) = document.get("project").and_then(toml::Value::as_table) else {
        return false;
    };
    if project
        .get("dynamic")
        .and_then(toml::Value::as_array)
        .is_some_and(|dynamic| dynamic.iter().any(|v| v.as_str() == Some("dependencies")))
        || project
            .get("dependencies")
            .is_some_and(|dependencies| dependencies.as_array().is_none())
        || project.get("optional-dependencies").is_some_and(|groups| {
            !groups.as_table().is_some_and(|table| {
                table.values().all(|items| {
                    items
                        .as_array()
                        .is_some_and(|items| items.iter().all(toml::Value::is_str))
                })
            })
        })
    {
        return false;
    }
    if let Some(dependencies) = project.get("dependencies").and_then(toml::Value::as_array) {
        // The current resolver models only a simple name + version range.
        // Admit no PEP 508 extras, environment markers, URL references, or
        // compound syntax that would be flattened during MGC add/remove.
        // (Resolver hiện chỉ mô hình tên + version range đơn giản; từ chối
        // extras/marker/URL/cú pháp ghép để không làm mất semantics.)
        if dependencies.iter().any(|dependency| {
            dependency
                .as_str()
                .is_none_or(|spec| parse_python_dependency(spec).is_err())
        }) {
            return false;
        }
    }
    if !supports_native_python_dependency_groups(&document) {
        return false;
    }

    // Do not select only PEP 621 dependencies when a second manager or
    // dependency-group mechanism may contribute packages to the environment.
    let tool = document.get("tool");
    let uv = tool.and_then(|v| v.get("uv"));
    let has_foreign_dependency_source = [
        tool.and_then(|v| v.get("poetry")),
        tool.and_then(|v| v.get("pdm")),
        tool.and_then(|v| v.get("pixi")),
        uv.and_then(|v| v.get("sources")),
        uv.and_then(|v| v.get("dev-dependencies")),
        uv.and_then(|v| v.get("dependency-groups")),
    ]
    .into_iter()
    .flatten()
    .any(|value| !value.as_table().is_some_and(toml::map::Map::is_empty));
    if has_foreign_dependency_source {
        return false;
    }

    let Ok(mut entries) = std::fs::read_dir(root) else {
        return false;
    };
    entries.all(|entry| {
        let Ok(entry) = entry else {
            return false;
        };
        let Some(name) = entry.file_name().into_string().ok() else {
            return false;
        };
        let name = name.to_ascii_lowercase();
        !matches!(
            name.as_str(),
            "uv.lock"
                | "poetry.lock"
                | "pdm.lock"
                | "pipfile"
                | "pipfile.lock"
                | "pixi.lock"
                | "conda-lock.yml"
                | "conda-lock.yaml"
                | "requirements.lock"
                | "requirements.txt"
                | "pylock.toml"
        ) && !(name.starts_with("requirements") && name.ends_with(".txt"))
            && !(name.starts_with("pylock.") && name.ends_with(".toml"))
    })
}

/// MGC models PEP 735's conventional `dev` group only. Other groups, group
/// includes, and non-simple PEP 508 requirements remain outside the native
/// mutation lane until their selection semantics can be preserved.
/// (MGC chỉ mô hình nhóm `dev`; nhóm khác/include/cú pháp chưa hỗ trợ phải
/// fail-closed để không làm sai ý nghĩa khai báo.)
pub fn supports_native_python_dependency_groups(document: &toml::Value) -> bool {
    let Some(groups) = document.get("dependency-groups") else {
        return true;
    };
    let Some(groups) = groups.as_table() else {
        return false;
    };
    groups.iter().all(|(name, value)| {
        name == "dev"
            && value.as_array().is_some_and(|dependencies| {
                dependencies.iter().all(|dependency| {
                    dependency
                        .as_str()
                        .is_some_and(|spec| parse_python_dependency(spec).is_ok())
                })
            })
    })
}
