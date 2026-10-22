//! Python package-manager module and isolated-build rules.
//! Rule module package manager Python và build isolation.

pub(in crate::run::implementation::dependency_policy::implementation::languages) fn denial_reason(
    tool: &str,
    args: &[String],
) -> Option<&'static str> {
    if args
        .windows(2)
        .any(|pair| pair[0] == "-m" && is_forbidden_package_module(&pair[1]))
    {
        return Some("Python package-manager modules are not allowed through mgc-exec");
    }
    if has_module(args, "ensurepip") {
        return Some("Python ensurepip is not allowed through mgc-exec");
    }
    if tool == "python"
        && args
            .windows(2)
            .any(|pair| pair[0] == "-m" && pair[1] == "build")
        && !args.iter().any(|arg| arg == "--no-isolation")
    {
        return Some(
            "Python build frontend must include --no-isolation to prevent dependency installation",
        );
    }
    None
}

fn has_module(args: &[String], module: &str) -> bool {
    args.windows(2)
        .any(|pair| pair[0] == "-m" && pair[1] == module)
}

fn is_forbidden_package_module(module: &str) -> bool {
    [
        "pip", "uv", "poetry", "pipenv", "pdm", "conda", "mamba", "hatch", "rye", "pixi",
    ]
    .iter()
    .any(|name| {
        module == *name
            || module
                .strip_prefix(name)
                .is_some_and(|suffix| suffix.starts_with('.'))
    })
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
