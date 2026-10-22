//! .NET restore, package tooling and MSBuild target rules.
//! Rule restore, package tooling và MSBuild target .NET.

pub(in crate::run::implementation::dependency_policy::implementation::languages) fn denial_reason(
    args: &[String],
    first: &str,
) -> Option<&'static str> {
    let has = |flag: &str| args.iter().any(|arg| arg == flag);
    match first {
        "restore" | "add" | "remove" => {
            Some(".NET dependency-management subcommands are not allowed through mgc-exec")
        }
        "tool" | "workload" | "nuget" | "msbuild" => {
            Some(".NET package/toolchain-management commands are not allowed through mgc-exec")
        }
        _ if requests_restore_target(args) => {
            Some("explicit MSBuild Restore targets are not allowed through mgc-exec")
        }
        "build" | "test" | "publish" | "run" | "pack" if !has("--no-restore") => {
            Some(".NET compile/test commands must include --no-restore")
        }
        _ => None,
    }
}

fn requests_restore_target(args: &[String]) -> bool {
    const TARGET_PREFIXES: &[&str] = &["-t:", "/t:", "-target:", "/target:", "--target="];

    args.iter().enumerate().any(|(index, arg)| {
        let target_value = TARGET_PREFIXES
            .iter()
            .find_map(|prefix| arg.strip_prefix(prefix))
            .or_else(|| {
                matches!(
                    arg.as_str(),
                    "-t" | "/t" | "-target" | "/target" | "--target"
                )
                .then(|| args.get(index + 1).map(String::as_str))
                .flatten()
            });

        target_value.is_some_and(|targets| {
            targets.split([';', ',']).any(|target| {
                target
                    .trim_matches(|ch| matches!(ch, '\'' | '"'))
                    .eq_ignore_ascii_case("restore")
            })
        })
    })
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
