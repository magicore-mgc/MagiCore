// Environment for native framework runtimes — môi trường runtime framework native.
// Native framework runtime environment.
use super::*;
use crate::commands::core::web::implementation::project_context::native_install_environment::native_install_env;

pub(in crate::commands::core::web::implementation) fn native_runtime_env(
    project_root: &Path,
    program: &Path,
) -> Result<Vec<(String, String)>> {
    let Some(name) = program.file_name().and_then(|name| name.to_str()) else {
        return Ok(Vec::new());
    };
    let mut env = native_install_env(project_root, name)?;
    if is_python_interpreter_name(name) {
        crate::commands::python_runtime::extend_native_python_env(&mut env, project_root, false)?;
    }
    Ok(env)
}
pub(in crate::commands::core::web::implementation) fn is_python_interpreter_name(
    name: &str,
) -> bool {
    let name = name.to_ascii_lowercase();
    let stem = name.strip_suffix(".exe").unwrap_or(&name);
    if stem == "python" {
        return true;
    }

    let Some(version) = stem.strip_prefix("python") else {
        return false;
    };
    let mut previous_was_dot = true;
    let mut saw_digit = false;
    for character in version.chars() {
        if character.is_ascii_digit() {
            previous_was_dot = false;
            saw_digit = true;
        } else if character == '.' && !previous_was_dot {
            previous_was_dot = true;
        } else {
            return false;
        }
    }
    saw_digit && !previous_was_dot
}
pub(in crate::commands::core::web::implementation) fn native_astro_update_check_env(
    program: &Path,
    compat: &crate::commands::compat::CompatMode,
) -> Option<(String, String)> {
    let is_astro = program
        .file_stem()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.eq_ignore_ascii_case("astro"));
    (is_astro && matches!(compat, crate::commands::compat::CompatMode::Native))
        .then(|| ("ASTRO_DISABLE_UPDATE_CHECK".to_string(), "true".to_string()))
}
