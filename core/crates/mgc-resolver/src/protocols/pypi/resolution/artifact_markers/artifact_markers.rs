//! Build stable marker strings for the selected artifact.
//! Tạo marker ổn định cho artifact đã chọn.

use crate::protocols::pypi::{PypiFile, wheel_tags};

pub(in crate::protocols::pypi::resolution) fn project_artifact_markers(
    file: &PypiFile,
) -> Vec<String> {
    let mut markers = Vec::new();
    if file.packagetype == "bdist_wheel" {
        let (python, abi, platform) = wheel_tags(&file.filename);
        markers.push(format!("wheel:{python}-{abi}-{platform}"));
    } else {
        markers.push(format!("sdist:{}", file.filename));
    }
    if let Some(required_python) = &file.requires_python {
        markers.push(format!("requires-python:{required_python}"));
    }
    markers
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
