use anyhow::Result;
use std::ffi::OsString;
use std::path::{Component, Path, PathBuf};

use super::super::super::node_command::{node_bin_args, node_bin_args_plain, node_runner};
use super::super::types::BuildLaunch;

pub(crate) fn map_framework_build_script(
    root: &Path,
    tokens: &[&str],
    compat_runner: Option<&str>,
) -> Result<Option<BuildLaunch>> {
    // Compat lane (P0-1): bun/deno build scripts spawn the rival runtime
    // DIRECTLY — only reachable when the compat gate above already passed
    // (native mode fails earlier at gate_runtime_spawn). No bun.pm usage.
    // Lane compat: script build bun/deno spawn runtime đối thủ TRỰC TIẾP —
    // chỉ đến được đây khi cổng compat phía trên đã pass.
    if let ["bun", rest @ ..] = tokens {
        return Ok(Some((
            PathBuf::from("bun"),
            rest.iter().map(OsString::from).collect(),
            vec![],
        )));
    }
    if let ["deno", rest @ ..] = tokens {
        return Ok(Some((
            PathBuf::from("deno"),
            rest.iter().map(OsString::from).collect(),
            vec![],
        )));
    }
    // Spec rev4 §7/§9: a declared compat runner maps to a direct launch of
    // itself (same shape as the bun/deno arms above); the exec layer applies
    // compat-audit instead of native-deny. Undeclared PM tokens fall through
    // to Ok(None) → unsupported-chain handling by the caller.
    // (Spec rev4 §7/§9: runner compat đã khai báo map thành launch trực tiếp
    // chính nó (giống arm bun/deno trên); exec layer áp compat-audit thay vì
    // native-deny. Token PM không khai báo rơi xuống Ok(None).)
    if let [first, rest @ ..] = tokens
        && mgc_exec::prelude::compat_runner_allows(first, compat_runner)
    {
        return Ok(Some((
            PathBuf::from(first),
            rest.iter().map(OsString::from).collect(),
            vec![],
        )));
    }
    let launch = match tokens {
        ["node", "--check", source] => {
            let Some(source_path) = resolve_project_source_file(root, source) else {
                return Ok(None);
            };
            (
                node_runner(),
                vec![OsString::from("--check"), source_path.into_os_string()],
                vec![],
            )
        }
        [tool @ ("tsc" | "vue-tsc"), rest @ ..] => {
            // `tsc` and `vue-tsc` load their implementation with a relative
            // `require('../lib/tsc.js')`. Under `--preserve-symlinks` that
            // resolves from the `.bin` link instead of the real file and the
            // compiler cannot start. npm and npx run them with realpath
            // resolution, so MagiCore does the same for these two.
            // `tsc` và `vue-tsc` nạp phần cài đặt bằng `require('../lib/tsc.js')`
            // tương đối. Dưới `--preserve-symlinks`, require resolve từ link
            // `.bin` chứ không từ file thật nên compiler không khởi động được.
            // npm và npx chạy chúng với realpath, nên MagiCore cũng vậy.
            (
                node_runner(),
                node_bin_args_plain(root, tool, rest)?,
                vec![],
            )
        }
        ["nest", "build", rest @ ..] => (
            node_runner(),
            node_bin_args(root, "nest", &["build"])?
                .into_iter()
                .chain(rest.iter().map(OsString::from))
                .collect(),
            vec![],
        ),
        ["vite", "build"] => (
            node_runner(),
            node_bin_args(root, "vite", &["build"])?,
            vec![],
        ),
        ["vite", "build", rest @ ..] => (
            node_runner(),
            node_bin_args(root, "vite", &["build"])?
                .into_iter()
                .chain(rest.iter().map(OsString::from))
                .collect(),
            vec![],
        ),
        ["next", "build"] => (
            node_runner(),
            node_bin_args(root, "next", &["build"])?,
            vec![],
        ),
        ["next", "build", rest @ ..] => (
            node_runner(),
            node_bin_args(root, "next", &["build"])?
                .into_iter()
                .chain(rest.iter().map(OsString::from))
                .collect(),
            vec![],
        ),
        ["nuxt", "build"] => (
            node_runner(),
            node_bin_args(root, "nuxt", &["build"])?,
            vec![
                (
                    OsString::from("NUXT_TELEMETRY_DISABLED"),
                    OsString::from("1"),
                ),
                (
                    OsString::from("NUXT_TELEMETRY_CONSENT"),
                    OsString::from("0"),
                ),
            ],
        ),
        ["astro", "build"] => (
            node_runner(),
            node_bin_args(root, "astro", &["build"])?,
            vec![],
        ),
        ["remix", "vite:build"] => (
            node_runner(),
            node_bin_args_plain(root, "remix", &["vite:build"])?,
            vec![],
        ),
        ["remix", "build"] => (
            node_runner(),
            node_bin_args_plain(root, "remix", &["build"])?,
            vec![],
        ),
        ["react-router", "build"] => (
            node_runner(),
            node_bin_args_plain(root, "react-router", &["build"])?,
            vec![],
        ),
        ["ng", "build"] => (
            node_runner(),
            node_bin_args_plain(root, "ng", &["build"])?,
            vec![
                (OsString::from("NG_CLI_ANALYTICS"), OsString::from("false")),
                (OsString::from("CI"), OsString::from("1")),
            ],
        ),
        _ => return Ok(None),
    };

    Ok(Some(launch))
}

fn resolve_project_source_file(root: &Path, source: &str) -> Option<PathBuf> {
    // Build syntax checks may inspect only regular files contained in this project.
    // Kiểm tra cú pháp chỉ được đọc file thường nằm bên trong project hiện tại.
    let relative = Path::new(source);
    if relative.as_os_str().is_empty()
        || relative.is_absolute()
        || relative
            .components()
            .any(|component| !matches!(component, Component::Normal(_) | Component::CurDir))
    {
        return None;
    }

    let canonical_root = root.canonicalize().ok()?;
    let canonical_source = canonical_root.join(relative).canonicalize().ok()?;
    (canonical_source.is_file() && canonical_source.starts_with(&canonical_root))
        .then_some(canonical_source)
}
