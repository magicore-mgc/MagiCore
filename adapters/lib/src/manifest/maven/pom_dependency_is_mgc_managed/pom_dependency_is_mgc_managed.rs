//! Maven ownership predicate.
//! Điều kiện ownership Maven.

/// Only the default compile/runtime, non-optional dependency set is MGC-owned.
/// MGC chỉ sở hữu dep compile/runtime mặc định, không optional.
pub(crate) fn pom_dependency_is_mgc_managed(dependency: &str) -> bool {
    let scope = dependency
        .split("<scope>")
        .nth(1)
        .and_then(|value| value.split("</scope>").next())
        .map(str::trim);
    let optional = dependency
        .split("<optional>")
        .nth(1)
        .and_then(|value| value.split("</optional>").next())
        .map(str::trim);
    scope.is_none_or(|scope| matches!(scope, "compile" | "runtime")) && optional != Some("true")
}
