//! Canonical compiler options and compilation cache identity.
//! Compiler options chuẩn hóa và danh tính cache biên dịch.

use super::super::tsconfig::tsconfig_digest;

/// Compiler identity + version for the compilation key (P0-1 audit
/// vòng-4). esbuild-rs 0.13.8 exposes NO runtime version API, so the
/// version is pinned to the workspace dependency: bumping `esbuild-rs` in
/// Cargo.toml MUST bump this constant together — the cache then
/// self-invalidates (new compiler version → new key → fresh compile).
///
/// (Danh tính + phiên bản compiler cho compilation key (P0-1 audit
/// vòng-4). esbuild-rs 0.13.8 KHÔNG có API runtime version nên version
/// ghim theo dependency workspace: nâng `esbuild-rs` trong Cargo.toml thì
/// PHẢI nâng hằng này theo — cache tự vô hiệu (compiler mới → key mới →
/// biên dịch lại).)
pub(in crate::bundler::dev_server::implementation) const COMPILER_IDENTITY: &str = "esbuild-rs";
pub(in crate::bundler::dev_server::implementation) const COMPILER_VERSION: &str = "0.13.8";

/// Canonical compile-options JSON (P0-1): every esbuild option the dev
/// server's transpile output depends on, serialized deterministically
/// (serde_json::json! object → BTreeMap under `preserve_order` OFF →
/// sorted keys) so the options digest is byte-stable across processes and
/// restarts. Change a pipeline option → change this JSON → the cache key
/// changes → entries recompile (no stale cross-option serving).
///
/// (JSON options biên dịch chuẩn hóa (P0-1): mọi option esbuild mà output
/// transpile của dev server phụ thuộc, serialize tất định (object
/// serde_json::json! → BTreeMap khi tắt `preserve_order` → key đã sort)
/// để digest ổn định từng byte giữa các process và restart. Đổi option
/// pipeline → đổi JSON này → key đổi → entry biên dịch lại (không phục vụ
/// stale chéo option).)
/// Effective compilation context (Hướng B, P0-2 Tech Lead vòng-6/7): the
/// cache key must capture EVERYTHING the dev server's transpile output
/// depends on BEYOND the source bytes — the project's tsconfig semantics
/// (jsx, target, decorators, useDefineForClassFields...), the project
/// root (sourcemap paths, resolve base) and the source's logical path.
/// Without this, two projects sharing the global compiled cache could
/// CROSS-HIT: same source bytes + different tsconfig → one project
/// receives the other's output. The context is embedded into the
/// canonical options JSON that feeds `CompilationKey::new` (its digest
/// becomes part of the key), so a context change = a key change = a fresh
/// compile, never a stale cross-project serve.
///
/// Ngữ cảnh biên dịch hiệu dụng (Hướng B, P0-2): cache key phải chứa MỌI
/// thứ mà output transpile phụ thuộc NGOÀI bytes source — semantics
/// tsconfig của project (jsx, target, decorators, useDefineForClassFields...),
/// root project (path sourcemap, resolve base) và path logic của source.
/// Thiếu nó, 2 project dùng chung compiled cache toàn cục có thể CROSS-HIT:
/// cùng bytes source + tsconfig khác → project này nhận output của project
/// kia. Ngữ cảnh được nhúng vào JSON options chuẩn hóa feeding
/// `CompilationKey::new` (digest của nó thành một phần key), nên đổi ngữ
/// cảnh = đổi key = biên dịch lại, không bao giờ serve stale chéo project.
pub(in crate::bundler::dev_server::implementation) fn canonical_compile_options(
    project_root: &std::path::Path,
    source_path: &std::path::Path,
) -> std::result::Result<String, super::super::tsconfig::TsconfigDigestError> {
    // The field set must mirror compiler/compiler.rs builder assignments or the cache key can drift.
    // Tập field phải phản chiếu gán builder tại compiler/compiler.rs để key cache không lệch.
    // Fail-closed tsconfig (Gate 11-C, vòng-11): an unreadable tsconfig
    // must FAIL the options build — the compile never runs under
    // unknown semantics.
    // (Tsconfig fail-closed: tsconfig không đọc được phải FAIL việc dựng
    // options — compile không bao giờ chạy dưới semantics không xác định.)
    let tsconfig = tsconfig_digest(project_root)?;
    let opts = serde_json::json!({
        "bundle": false,
        "format": "esmodule",
        "platform": "browser",
        "resolve_extensions": [".tsx", ".ts", ".jsx", ".js"],
        "source_map": "inline",
        "write": false,
        // ── EffectiveCompilationContext (Hướng B, P0-2) ──
        "project_root_digest": project_root_digest(project_root),
        "normalized_source_path": normalized_source_path(project_root, source_path),
        "tsconfig_digest": tsconfig.key_fragment(),
    });
    Ok(serde_json::to_string(&opts).unwrap_or_default())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;

/// Digest of the project root's canonical path — separates the cache
/// namespaces of two different projects (their sourcemaps embed
/// root-relative paths, so their outputs are NOT interchangeable even for
/// identical sources).
/// (Digest của path chuẩn hóa của project root — tách namespace cache
/// của 2 project khác nhau (sourcemap của chúng nhúng path theo root,
/// nên output KHÔNG hoán đổi được kể cả khi source giống hệt).)
fn project_root_digest(root: &std::path::Path) -> String {
    let canonical = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());
    blake3::Hasher::new()
        .update(canonical.to_string_lossy().as_bytes())
        .finalize()
        .to_hex()
        .to_string()
}

/// Logical (project-relative) source path — stable across machine
/// checkouts, unique within a project. Two sources at different logical
/// paths are different compilations even with equal bytes (they produce
/// different sourcemap `sources` entries).
/// (Path logic (theo project) của source — ổn định qua các máy checkout,
/// duy nhất trong project. Hai source khác path logic là 2 lần biên dịch
/// khác nhau kể cả khi bytes bằng nhau (chúng sinh `sources` trong
/// sourcemap khác nhau).)
fn normalized_source_path(root: &std::path::Path, source: &std::path::Path) -> String {
    // Prefer the project-relative logical path; fall back to the file
    // name alone (source outside the root — caller wiring), then the raw
    // path as a last resort. All three forms are deterministic for a
    // given (project, file) pair, which is what the key requires.
    // (Ưu tiên path logic theo project; fallback chỉ tên file (source nằm
    // ngoài root — wiring của caller), cuối cùng là path thô. Cả ba dạng
    // đều tất định cho một cặp (project, file) — đúng cái key cần.)
    if let Ok(p) = source.strip_prefix(root) {
        return p.to_string_lossy().into_owned();
    }
    if let Some(name) = source.file_name() {
        return name.to_string_lossy().into_owned();
    }
    source.to_string_lossy().into_owned()
}
