//! Dependency lifecycle event classification and pre/post hook veto.
//! Nhận diện event dependency và chặn hook ở cả phase pre/post.

use anyhow::{Result, bail};

use super::super::config::HooksConfig;

pub(in crate::hooks::owners) fn dependency_event_verb(event: &str) -> Option<&str> {
    let verb = event
        .strip_prefix("pre-")
        .or_else(|| event.strip_prefix("post-"))
        .unwrap_or(event);
    matches!(verb, "install" | "add" | "remove" | "update" | "publish").then_some(verb)
}

// `deno` rides along: rival JS runtimes must never execute on dependency
// events, matching the Install-scope guard in mgc-exec.
// (Kèm `deno`: runtime JS đối thủ không bao giờ chạy trên event dependency,
// khớp cổng Install-scope trong mgc-exec.)

pub(in crate::hooks::owners) fn ensure_dependency_event_hooks_allowed(
    cfg: &HooksConfig,
    event: &str,
) -> Result<()> {
    // Arbitrary hook executables can spawn package managers through wrappers,
    // interpreters, or project scripts, so a tool-name denylist is not a
    // security boundary. Refuse both phases before dependency mutation starts.
    // (Hook tùy ý có thể gọi package manager qua wrapper/interpreter/script,
    // nên denylist tên tool không đủ an toàn. Chặn cả hai phase trước mutation.)
    if let Some(verb) = dependency_event_verb(event) {
        let pre_event = format!("pre-{verb}");
        let post_event = format!("post-{verb}");
        let has_dependency_hook = [event, pre_event.as_str(), post_event.as_str()]
            .iter()
            .filter_map(|name| cfg.hooks.get(*name))
            .flatten()
            .any(|command| !command.trim().is_empty());
        if has_dependency_hook {
            bail!(
                concat!(
                    "user-defined hooks are disabled for dependency lifecycle event '{0}' ",
                    "because they can execute external package managers; remove the pre/post ",
                    "hook from mgc.hooks.toml or the user hooks file"
                ),
                verb
            );
        }
    }

    Ok(())
}
