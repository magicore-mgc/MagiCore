use anyhow::Result;
use std::path::Path;

use super::super::script::map_framework_build_script;
use super::super::types::BuildLaunch;

pub(crate) fn map_framework_build_chain(
    root: &Path,
    script: &str,
    compat: &crate::commands::compat::CompatMode,
    compat_runner: Option<&str>,
) -> Result<Option<Vec<BuildLaunch>>> {
    // Map every AND-linked command before executing any of them; never pass
    // shell operators to a framework or invoke a shell to interpret them.
    // Map toàn bộ chuỗi AND trước khi chạy; không đưa toán tử vào framework/shell.
    let mut launches = Vec::new();
    let invocations = mgc_exec::allowlist::parse_script_chain(script)?;
    let compound = invocations.len() > 1;
    for invocation in invocations {
        // Spec rev4 §7/§9: the declared `--compat-runner` manager skips the
        // rival-runtime gate; its launches run compat-audit downstream.
        // (Spec rev4 §7/§9: manager `--compat-runner` đã khai báo bỏ qua
        // cổng rival-runtime; launch của nó chịu compat-audit phía sau.)
        if !mgc_exec::prelude::compat_runner_allows(&invocation.program, compat_runner) {
            crate::commands::compat::gate_runtime_spawn(compat, &invocation.program)?;
        }
        let mut words = vec![invocation.program.as_str()];
        words.extend(invocation.args.iter().map(String::as_str));
        let Some((program, args, mut envs)) =
            map_framework_build_script(root, &words, compat_runner)?
        else {
            if compound {
                anyhow::bail!(
                    "unsupported command in framework build chain: {}",
                    invocation.program
                );
            }
            return Ok(None);
        };
        envs.extend(
            invocation
                .env
                .into_iter()
                .map(|(key, value)| (key.into(), value.into())),
        );
        launches.push((program, args, envs));
    }
    Ok(Some(launches))
}
