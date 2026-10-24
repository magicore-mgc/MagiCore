//! MagiCore-owned additions to a reused Python project manifest.
//! Phần do MagiCore sở hữu, thêm vào manifest project Python tái dùng.
//!
//! The framework source is copied verbatim from a local project or a registry
//! layer. What MagiCore owns is the test/build toolchain it promises for
//! `mgc test` and `mgc build`, so those pins are merged into `pyproject.toml`
//! here instead of being baked into any framework template.
//! Source framework được copy nguyên vẹn từ project local hoặc registry layer.
//! Phần MagiCore sở hữu là toolchain test/build mà `mgc test` và `mgc build`
//! cam kết, nên các pin đó được hợp nhất vào `pyproject.toml` ở đây thay vì
//! nằm sẵn trong bất kỳ template framework nào.

use std::path::Path;

use anyhow::Result;

/// Dev pins MagiCore installs for every Python project it scaffolds.
/// Pin dev MagiCore cài cho mọi project Python nó scaffold.
const PYTHON_DEV_PINS: [&str; 3] = ["pytest==9.1.1", "build==1.6.1", "setuptools==84.0.0"];

/// Ensure `pyproject.toml` declares MagiCore's PEP 735 dev group.
///
/// The merge is additive and idempotent: an existing `[dependency-groups]`
/// table keeps its entries, and only the missing pins are appended.
/// Bảo đảm `pyproject.toml` có nhóm dev PEP 735 của MagiCore. Hợp nhất theo
/// kiểu cộng dồn và idempotent: bảng `[dependency-groups]` có sẵn giữ nguyên
/// entry, chỉ thêm pin còn thiếu.
pub(in crate::scaffold::processor) fn ensure_python_dev_group(target: &Path) -> Result<()> {
    let path = target.join("pyproject.toml");
    let mut text = if path.is_file() {
        std::fs::read_to_string(&path)?
    } else {
        String::new()
    };

    if text.contains("[dependency-groups]") {
        let mut section = String::new();
        for pin in PYTHON_DEV_PINS {
            if !text.contains(pin) {
                section.push_str(&format!("    \"{pin}\",\n"));
            }
        }
        if section.is_empty() {
            return Ok(());
        }
        // Append the missing pins to the existing `dev` list.
        // Thêm pin còn thiếu vào danh sách `dev` có sẵn.
        if let Some(index) = text.find("dev = [") {
            let insert_at = text[index..]
                .find(']')
                .map(|offset| index + offset)
                .unwrap_or_else(|| text.len());
            text.insert_str(insert_at, &section);
        } else {
            text.push_str("\ndev = [\n");
            text.push_str(&section);
            text.push_str("]\n");
        }
        std::fs::write(&path, text)?;
        return Ok(());
    }

    if !text.is_empty() && !text.ends_with('\n') {
        text.push('\n');
    }
    text.push_str("\n[dependency-groups]\ndev = [\n");
    for pin in PYTHON_DEV_PINS {
        text.push_str(&format!("    \"{pin}\",\n"));
    }
    text.push_str("]\n");
    std::fs::write(&path, text)?;
    Ok(())
}
