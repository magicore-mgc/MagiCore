//! Build JavaScript payloads for dependency responses.
//! Tạo JavaScript payload cho response dependency.

use super::super::super::imports::js_string_literal;

pub(in crate::bundler::dev_server::implementation::dependency_route) fn css_injector(
    css: &str,
) -> String {
    let css_literal = js_string_literal(css);
    format!(
        r#"
// [MgDevServer] Injected package CSS
(function() {{
  const sheet = new CSSStyleSheet();
  sheet.replaceSync({css_literal});
  document.adoptedStyleSheets = [...document.adoptedStyleSheets, sheet];
}})();
"#
    )
}

pub(in crate::bundler::dev_server::implementation::dependency_route) fn missing_dependency_diagnostic(
    package: &str,
) -> String {
    let package_literal = js_string_literal(package);
    format!("console.error('[MgDevServer] Failed to bundle dependency:', {package_literal});")
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
