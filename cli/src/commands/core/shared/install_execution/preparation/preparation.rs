//! Parse the manifest, reuse or resolve its graph, and enforce install policy.
//! Parse manifest, tái dùng hoặc resolve graph và áp dụng install policy.

use super::*;

pub(crate) async fn prepare_install_execution(
    adapter: &dyn PackageAdapter,
    root: &Path,
    frozen: bool,
    add_cmd: Option<&str>,
) -> Result<InstallExecution> {
    let started_at = std::time::Instant::now();
    let spinner = create_spinner("  Reading project manifest...");
    let manifest = adapter.parse_manifest(root).await?;
    spinner.finish_and_clear();
    profile_install_mark("parse_manifest", started_at);

    let all_deps: Vec<_> = manifest.all_dependencies().collect();
    if all_deps.is_empty() && !adapter.resolves_implicit_dependencies(root)? {
        if frozen && add_cmd.is_some() {
            if let Some(graph) = load_locked_graph(root, adapter.name(), &manifest)? {
                return Ok(InstallExecution {
                    graph,
                    summary: mgc_types::adapter::InstallSummary::default(),
                    used_lockfile: true,
                });
            }
            let command = install_command_for_adapter(adapter);
            if root.join("mgc.lock").is_file() {
                return Err(crate::error::frozen_lock_mismatch(command));
            }
            return Err(crate::error::frozen_lock_missing(command));
        }
        if let Some(add_cmd) = add_cmd {
            info("No dependencies to install.");
            info(&format!(
                "Use '{} <package>' to add dependencies.",
                style_cmd(add_cmd)
            ));
        }
        return Ok(InstallExecution {
            graph: ResolvedGraph::empty(),
            summary: mgc_types::adapter::InstallSummary::default(),
            used_lockfile: false,
        });
    }

    // Resolve and replay must agree on registry requirements supplied by an SDK.
    // Replay dùng cùng dependency SDK như resolver, vẫn giữ manifest gốc cho mutation.
    let replay_manifest = adapter.lock_replay_manifest(root, &manifest)?;
    let loaded_lock = load_locked_graph(root, adapter.name(), &replay_manifest)?;
    let force_fresh_resolve =
        refresh_locked_graph_for_age_gate(adapter, root, loaded_lock.is_some(), frozen)?;
    let loaded_lock = if force_fresh_resolve {
        None
    } else {
        loaded_lock
    };
    let (graph, used_lockfile) = if let Some(graph) = loaded_lock {
        info("Using mgc.lock for install state.");
        profile_install_mark("load_locked_graph", started_at);
        (graph, true)
    } else {
        if frozen {
            let cmd = install_command_for_adapter(adapter);
            // Tamper/mismatch (lock present but not matching) fails
            // LOUDLY — silently re-resolving would bless a tampered lock
            // and rewrite it. Missing lock is a different error.
            if root.join("mgc.lock").is_file() {
                return Err(crate::error::frozen_lock_mismatch(cmd));
            }
            return Err(crate::error::frozen_lock_missing(cmd));
        }
        let spinner = create_spinner(&format!("  Resolving {} dependencies...", all_deps.len()));
        let resolve_started_at = std::time::Instant::now();
        // P0/F6: arm the age gate from THIS operation's project (never a
        // process-global first-wins); broken config fails the op here.
        adapter.arm_age_gate_for(root)?;
        let graph = adapter.resolve(&manifest).await?;
        spinner.finish_and_clear();
        profile_install_mark("resolve_graph", resolve_started_at);
        (graph, false)
    };
    enforce_audit_strict_policy(adapter, root, &graph).await?;
    profile_install_mark("prepare_install_execution_total", started_at);
    Ok(InstallExecution {
        graph,
        summary: mgc_types::adapter::InstallSummary {
            duration_ms: started_at.elapsed().as_millis() as u64,
            ..Default::default()
        },
        used_lockfile,
    })
}
