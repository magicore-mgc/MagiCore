//! Authentication policy helpers.
// Glob matching cùng access wrappers public.

/// Glob match cho scope/package: `*` (mọi thứ), `@scope/*` (prefix theo scope), còn lại so khớp chính xác
pub fn scope_matches(pattern: &str, package: &str) -> bool {
    match pattern {
        "*" | "**" => true,
        _ => {
            if let Some(prefix) = pattern.strip_suffix("/*") {
                package == prefix || package.starts_with(&format!("{}/", prefix))
            } else {
                pattern == package
            }
        }
    }
}
