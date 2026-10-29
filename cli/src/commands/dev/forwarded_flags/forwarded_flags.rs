//! Validate flags a dev script forwards to a Node-family runtime.
//! Kiểm flag mà script dev chuyển tiếp cho runtime họ Node.

use anyhow::Result;

/// JS runtimes whose flags the launcher policy models.
/// Runtime JS mà policy launcher mô hình hoá.
const NODE_FAMILY: &[&str] = &["node", "tsx", "bun", "deno"];

/// Framework CLIs that ultimately run on a JS runtime and forward their argv.
///
/// `next`, `vite`, `nuxt`, `astro`, `webpack`, `react-scripts` are Node programs;
/// a flag they do not recognise is not necessarily harmless — some (`--require`)
/// are read by Node itself before the framework sees argv.
/// (`next`/`vite`/`nuxt`/`astro`/`webpack`/`react-scripts` là chương trình Node;
/// flag chúng không hiểu chưa chắc vô hại — một số như `--require` do Node đọc
/// trước khi framework thấy argv.)
const NODE_FRAMEWORK_BINARIES: &[&str] = &[
    "next",
    "vite",
    "nuxt",
    "astro",
    "webpack",
    "react-scripts",
    "vue-cli-service",
    "nest",
    "ng",
    "remix",
];

/// Refuse a forwarded flag that the launcher policy would refuse for `node`.
///
/// The script's program selects the profile: a JS runtime uses its own runtime,
/// a framework CLI uses the Node profile because that is the process the flag
/// eventually reaches. Anything else is left alone — a `cargo`-driven script has
/// its own toolchain rules and this policy is not the place to guess.
/// (Program của script chọn profile: runtime JS dùng runtime riêng, CLI framework
/// dùng profile Node vì flag sẽ tới tiến trình đó. Còn lại để nguyên.)
pub fn validate_forwarded_runtime_flags(program: &str, args: &[&str]) -> Result<()> {
    use crate::commands::launcher_policy::{LauncherPolicy, Runtime};

    let profile = program
        .rsplit('/')
        .next()
        .unwrap_or(program)
        .trim_end_matches(".exe");
    let runtime = if NODE_FAMILY.contains(&profile) {
        match profile {
            "bun" => Runtime::Bun,
            "deno" => Runtime::Deno,
            "tsx" => Runtime::Tsx,
            _ => Runtime::Node,
        }
    } else if NODE_FRAMEWORK_BINARIES.contains(&profile) {
        Runtime::Node
    } else {
        return Ok(());
    };
    LauncherPolicy::dev_server(runtime).validate_args(args)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
