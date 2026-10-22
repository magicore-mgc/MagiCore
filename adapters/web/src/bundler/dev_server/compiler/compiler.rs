//! Native esbuild runner and safe error output — esbuild native và output lỗi an toàn.
//! Performs transpilation without bundling.

use std::path::Path;

use super::super::imports::js_string_literal;
use super::CompilerDiagnostics;

pub(in crate::bundler::dev_server::implementation) async fn compile_transpiled_source(
    file_path: &Path,
    project_root: &Path,
) -> Result<String, CompilerDiagnostics> {
    let working_dir = project_root.to_string_lossy().to_string();
    let mut builder = esbuild_rs::BuildOptionsBuilder::new();
    builder.entry_points = vec![file_path.to_string_lossy().to_string()];
    builder.bundle = false; // Transpile only; do not bundle. / Chỉ transpile, không bundle.
    builder.abs_working_dir = working_dir;
    // Honor the project's tsconfig.json (Hướng B, P0-2): jsx mode, target,
    // decorators, useDefineForClassFields... must come from the PROJECT,
    // not from a fixed default — `mgc dev` respects the project's config.
    // The tsconfig CONTENT digest already sits in the cache key via
    // `canonical_compile_options`, so any tsconfig change = new key =
    // fresh compile (no stale cross-project hit can occur).
    // (Tôn trọng tsconfig.json của project (Hướng B, P0-2): jsx mode,
    // target, decorators, useDefineForClassFields... phải đến từ PROJECT,
    // không phải default cố định — `mgc dev` tôn trọng cấu hình project.
    // Digest NỘI DUNG tsconfig đã nằm trong cache key qua
    // `canonical_compile_options`, nên mọi thay đổi tsconfig = key mới =
    // biên dịch lại (không thể xảy ra stale hit chéo project).)
    let tsconfig_path = project_root.join("tsconfig.json");
    if tsconfig_path.exists() {
        builder.tsconfig = tsconfig_path.to_string_lossy().to_string();
    }
    builder.platform = esbuild_rs::Platform::Browser;
    builder.format = esbuild_rs::Format::ESModule;
    builder.source_map = esbuild_rs::SourceMap::Inline;
    builder.write = false;
    builder.resolve_extensions = vec![
        ".tsx".to_string(),
        ".ts".to_string(),
        ".jsx".to_string(),
        ".js".to_string(),
    ];

    let options = builder.build();
    let result = esbuild_rs::build(options).await;

    if !result.errors.as_slice().is_empty() {
        let msgs: Vec<String> = result
            .errors
            .as_slice()
            .iter()
            .map(|error| error.to_string())
            .collect();
        return Err(CompilerDiagnostics::new(msgs.join("\n")));
    }

    // Output selection (P0, vòng-11 — found by the WS HMR evidence test):
    // esbuild-rs 0.13.8 with `write=false` reports the single output
    // entry's path as `<stdout>` — the old filter
    // (`path.ends_with(".js")`) matched NOTHING, `raw_js` fell to the
    // empty default, and EVERY transpiled module was served as EMPTY JS
    // (the compiled cache stored empty payloads too). Prefer a real
    // `.js` output entry; with none, take the FIRST output —
    // bundle=false + one entry point means the first output IS the
    // transpiled module (verified by probe: exactly 1 output, correct
    // JS body, path `<stdout>`).
    // (Chọn output (P0 vòng-11 — test bằng chứng WS HMR tìm ra):
    // esbuild-rs 0.13.8 với `write=false` báo path của output đơn là
    // `<stdout>` — filter cũ (`ends_with(".js")`) KHÔNG khớp gì, `raw_js`
    // rơi về mặc định rỗng, và MỌI module transpile được phục vụ là JS
    // RỖNG (compiled cache cũng lưu payload rỗng). Ưu tiên entry `.js`
    // thật; không có thì lấy output ĐẦU — bundle=false + 1 entry point
    // nghĩa output đầu chính là module đã transpile (probe xác minh:
    // đúng 1 output, body JS đúng, path `<stdout>`).)
    let raw_js = result
        .output_files
        .as_slice()
        .iter()
        .find(|f| f.path.as_str().ends_with(".js"))
        .or_else(|| result.output_files.as_slice().first())
        .map(|f| f.data.as_str().to_string())
        .unwrap_or_default();

    Ok(raw_js)
}

pub(in crate::bundler::dev_server::implementation) fn compile_error_body(
    error_message: &str,
) -> String {
    let err_esc = js_string_literal(error_message);
    format!(
        r#"
const __err = {err_esc};
console.error('[MgDevServer] Build Error:', __err);
const el = document.getElementById('__mgc-error') || document.createElement('div');
el.id = '__mgc-error';
el.style = 'position:fixed;top:0;left:0;right:0;background:#1a0000;color:#ff6b6b;padding:20px;font-family:monospace;white-space:pre;z-index:99999;border-bottom:2px solid #f00';
el.textContent = '[MgDevServer Build Error]\n' + __err;
document.body?.prepend(el);
"#
    )
}
