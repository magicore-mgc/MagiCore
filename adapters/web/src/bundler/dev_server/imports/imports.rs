//! Dependency import rewriting and safe JavaScript string encoding.
//! Rewrite import dependency và mã hóa JavaScript string an toàn.

// Regex-based import rewriter for static and dynamic ESM imports.
// Bộ rewrite regex khớp cả import ESM tĩnh lẫn động.
// Example: import x from 'react' → import x from '/@magicore/deps/react'.
// Ví dụ: import('react') → import('/@magicore/deps/react').
static BARE_IMPORT_RE: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();

fn bare_import_re() -> &'static regex::Regex {
    BARE_IMPORT_RE.get_or_init(|| {
        // Matches the supported package-import forms.
        // Khớp các dạng package import được hỗ trợ.
        //   from 'pkg'          from "pkg"
        //   import 'pkg'        import "pkg"
        //   import('pkg')       import("pkg")
        //   export from 'pkg'
        // Relative, absolute and already rewritten paths are filtered by rewrite_imports.
        // Đường dẫn tương đối, tuyệt đối và đã rewrite được lọc trong rewrite_imports.
        regex::Regex::new(
            r#"(?P<keyword>from|import(?:\s*\()|import|export\s+(?:\*|(?:\{[^}]*\}))\s+from)\s*['"](?P<pkg>[^'"]+)['"]"#
        ).expect("invalid bare import regex")
    })
}

/// Rewrite bare imports in JS/TS output to /@magicore/deps/ paths.
/// Viết lại bare import trong output JS/TS thành path /@magicore/deps/.
pub(in crate::bundler::dev_server::implementation) fn rewrite_imports(js: &str) -> String {
    let re = bare_import_re();
    re.replace_all(js, |caps: &regex::Captures| {
        let keyword = &caps["keyword"];
        let pkg = &caps["pkg"];
        // Keep relative, absolute and already rewritten dependency paths unchanged.
        // Giữ nguyên path tương đối, tuyệt đối và dependency đã rewrite.
        if pkg.starts_with("./")
            || pkg.starts_with("../")
            || pkg.starts_with('/')
            || pkg.starts_with("@magicore/")
        {
            return caps
                .get(0)
                .map(|m| m.as_str().to_string())
                .unwrap_or_default();
        }
        // Preserve scoped package segments while changing only the URL prefix.
        // Giữ các segment scoped package và chỉ đổi prefix URL.
        // Example: @tanstack/react-query → /@magicore/deps/@tanstack/react-query.
        // Ví dụ: @tanstack/react-query → /@magicore/deps/@tanstack/react-query.
        let is_dynamic = keyword.starts_with("import(");
        if is_dynamic {
            format!("import('/@magicore/deps/{}')", pkg)
        } else {
            format!("{} '/@magicore/deps/{}'", keyword.trim_end(), pkg)
        }
    })
    .to_string()
}

/// Embed an arbitrary runtime string as a JS STRING LITERAL (P1-3,
/// fresh-context review 2026-09-15): dependency CSS and compiler errors
/// used to ride inside a JS template literal with a hand-rolled escape
/// (backtick-only) — a CSS payload containing `${...}` became a live JS
/// EXPRESSION in the browser page (semi-trusted npm dependency → code
/// execution). Instead of trying to out-escape the template-literal
/// grammar, the payload is embedded as a JSON string literal:
/// `serde_json::to_string` produces a double-quoted JSON string, which
/// is a valid JS string literal by construction (JSON escaping covers
/// `"`, `\`, control chars — no terminator or interpolation sequence can
/// survive). The string then flows into `replaceSync(...)` /
/// `textContent` as DATA, never as code.
/// (Nhúng chuỗi runtime tùy ý thành STRING LITERAL JS (P1-3): CSS
/// dependency và lỗi compiler từng nằm trong template literal JS với
/// escape tự chế (chỉ backtick) — payload CSS chứa `${...}` thành BIỂU
/// THỨC JS chạy trong trang browser. Thay vì đua escape với ngữ pháp
/// template literal, payload được nhúng dạng JSON string literal:
/// `serde_json::to_string` ra chuỗi JSON trích dẫn kép — hợp lệ là JS
/// string literal theo cấu trúc (JSON escape phủ `"`, `\`, control
/// chars — không chuỗi kết thúc hay nội suy nào sống sót). Chuỗi rồi
/// chảy vào replaceSync/textContent như DỮ LIỆU, không bao giờ là code.)
pub(crate) fn js_string_literal(s: &str) -> String {
    serde_json::to_string(s).unwrap_or_else(|_| "\"\"".to_string())
}

#[cfg(test)]
#[path = "test/imports.rs"]
mod tests;
