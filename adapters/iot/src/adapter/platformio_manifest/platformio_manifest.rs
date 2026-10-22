//! Parses only modeled PlatformIO env lib_deps. URL, git, file, symlink or otherwise unmodelable entries continue to return an error rather than a fabricated empty graph. — Chỉ parse lib_deps của env PlatformIO mà MGC mô hình được. URL, git, file, symlink hoặc entry không mô hình được tiếp tục trả lỗi thay vì tạo graph rỗng giả.

use std::path::Path;

use mgc_types::{Ecosystem, Manifest, MgResult};
/// Parse `lib_deps` from `platformio.ini` `[env:*]` sections (read-only —
/// pio owns the file). Entries are `owner/name[@req]`, comma- or
/// newline-separated, continuations indented. Anything mgc cannot model
/// (URLs, `file://`, `symlink://`) fails CLOSED naming the line — a
/// skipped dep would fake an empty graph (the old parser returned NO
/// deps at all, so `list` lied and the add re-read check could never
/// pass).
/// (Đọc `lib_deps` từ platformio.ini (chỉ đọc); dòng không mô hình được
/// thì lỗi rõ, không bỏ qua âm thầm.)
pub(in crate::adapter) fn parse_platformio_manifest(project_root: &Path) -> MgResult<Manifest> {
    let content = std::fs::read_to_string(project_root.join("platformio.ini"))
        .map_err(|e| mgc_types::MgError::Other(format!("read platformio.ini: {e}")))?;
    let name = project_root
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "firmware".to_string());
    let mut manifest = Manifest::new(&name, Ecosystem::Iot);
    let flush = |buf: &mut String, manifest: &mut Manifest| -> MgResult<()> {
        for raw in buf.split([',', '\n']) {
            let spec = raw.trim();
            if spec.is_empty() || spec.starts_with([';', '#']) {
                continue;
            }
            if spec.contains("://") || spec.starts_with("git@") || spec.starts_with("file:") {
                return Err(mgc_types::MgError::Other(format!(
                    "platformio.ini lib_deps entry '{spec}' is not modelable (URL/file/symlink dep) — manage it with pio directly; mgc refuses to fake the graph"
                )));
            }
            let dep = mgc_types::DependencySpec::parse(spec).map_err(|e| {
                mgc_types::MgError::Other(format!("parse platformio.ini lib_deps '{spec}': {e}"))
            })?;
            manifest.add_dep(dep, false, false, false);
        }
        buf.clear();
        Ok(())
    };
    let mut in_env = false;
    let mut collecting = false;
    let mut buf = String::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') {
            flush(&mut buf, &mut manifest)?;
            collecting = false;
            in_env = trimmed.starts_with("[env:");
            continue;
        }
        if !in_env {
            continue;
        }
        if collecting && (line.starts_with([' ', '\t']) || trimmed.is_empty()) {
            if !trimmed.is_empty() {
                buf.push('\n');
                buf.push_str(trimmed);
            }
            continue;
        }
        if collecting {
            flush(&mut buf, &mut manifest)?;
            collecting = false;
        }
        if let Some((key, value)) = trimmed.split_once('=')
            && key.trim() == "lib_deps"
        {
            buf.push_str(value.trim());
            collecting = true;
        }
    }
    flush(&mut buf, &mut manifest)?;
    Ok(manifest)
}
