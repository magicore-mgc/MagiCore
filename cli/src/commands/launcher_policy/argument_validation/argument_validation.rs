//! Ordered checks for runtime flags and Deno permissions.
//! Kiểm tra có thứ tự cho runtime flags và Deno permissions.

use anyhow::Result;

use super::super::{LauncherPolicy, Runtime, error::LauncherPolicyError, runtime};

/// Whether `arg` uses `flag` as a flag, not as a filename prefix.
///
/// Prefix matching was both too wide and too narrow: `-econsole.log(1)` and
/// `-experimental.ts` matched `-e` (one dangerous, one a legitimate script path),
/// while bundled shorts like `-ep` matched nothing.
///
/// Flags are matched as: the exact token (`-e`, `--eval`); a long flag with a
/// value (`--eval=code`, `--import=./x.mjs`); or a bundle of short options whose
/// first letter is the flag (`-ep`). Positional arguments after `--` are the
/// script path by definition and are never treated as flags.
/// (Prefix matching vừa quá rộng vừa quá hẹp: `-experimental.ts` khớp `-e`, còn
/// `-ep` không khớp gì. Nay: token chính xác, `--flag=value`, và cụm short.
/// Arg sau `--` là script path nên không bị coi là flag.)
fn uses_flag(arg: &str, flag: &str) -> bool {
    if arg == flag {
        return true;
    }
    if flag.starts_with("--") {
        // `--eval=code`: the destructive part is the value, so the value must be
        // refused rather than accepted as a distinct flag.
        // (`--eval=code`: phần nguy hiểm là giá trị.)
        return arg
            .strip_prefix(flag)
            .is_some_and(|rest| rest.starts_with('='));
    }
    // Bundled short options: `-ep` is `-e` + `-p`. Only single-letter flags can
    // bundle, and the flag must be the FIRST letter so `-pe` is reported as `-p`.
    // (Cụm short: `-ep` = `-e` + `-p`; flag phải là chữ đầu.)
    if flag.len() == 2 && flag.starts_with('-') {
        return arg
            .strip_prefix(flag)
            .is_some_and(|rest| !rest.is_empty() && !rest.starts_with('-'));
    }
    false
}

pub(in crate::commands::launcher_policy) fn validate_args(
    policy: &LauncherPolicy,
    args: &[&str],
) -> Result<()> {
    let mut positional = false;
    for arg in args {
        // Everything after `--` is a positional argument (the script path), so a
        // file named `-e.ts` can still be run explicitly.
        // (Sau `--` là positional; file tên `-e.ts` vẫn chạy được khi chỉ định rõ.)
        if *arg == "--" {
            positional = true;
            continue;
        }
        if positional {
            continue;
        }
        for flag in runtime::dangerous_flags(policy.runtime) {
            let flag = *flag;
            if uses_flag(arg, flag) {
                return Err(LauncherPolicyError::DangerousFlag {
                    runtime: policy.runtime.name(),
                    flag,
                }
                .into());
            }
        }
    }

    if matches!(policy.runtime, Runtime::Deno) && !policy.allow_dangerous_permissions {
        for arg in args {
            if *arg == "--" {
                break;
            }
            for permission in runtime::dangerous_permissions(policy.runtime) {
                let permission = *permission;
                if uses_flag(arg, permission) {
                    return Err(LauncherPolicyError::DangerousPermission {
                        runtime: policy.runtime.name(),
                        permission,
                    }
                    .into());
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
