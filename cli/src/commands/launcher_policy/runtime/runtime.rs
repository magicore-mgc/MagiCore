//! Runtime identity and ordered dangerous-argument profiles.
//! Định danh runtime và profile argument nguy hiểm có thứ tự.

/// Runtime types whose launcher arguments are checked by this policy.
/// Các runtime có arguments launcher được policy kiểm tra.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Runtime {
    Bun,
    Deno,
    Node,
    /// `tsx` is a Node loader wrapper: `tsx -e "<code>"` forwards to
    /// `node -e`, so it needs Node's flags profile, not an empty one.
    /// (`tsx` là wrapper Node loader: `tsx -e` chuyển tiếp `node -e`, nên cần
    /// profile flag của Node, không phải profile rỗng.)
    Tsx,
}

impl Runtime {
    pub fn name(&self) -> &'static str {
        match self {
            Runtime::Bun => "bun",
            Runtime::Deno => "deno",
            Runtime::Node => "node",
            Runtime::Tsx => "tsx",
        }
    }
}

// Every spelling that makes the runtime execute code the caller did not name as a
// script path.
//
// The Node list previously held only the four inline-eval spellings, so
// `node --import ./loader.mjs app.js` and `node --require evil app.js` ran
// arbitrary modules through both `mgc dev` and `mgc run`. `mgc-exec` already
// classifies `-r/--require/--import/--loader` as value flags for the same reason
// (`run/runtime_invocation/node/node.rs`), so the two policies disagreed about
// what is dangerous — this list now matches that classification.
// (Danh sách cũ chỉ có 4 spelling inline-eval, nên `node --import ./loader.mjs`
// và `node --require evil` chạy module tuỳ ý qua `mgc dev` lẫn `mgc run`.
// `mgc-exec` đã xếp chúng vào nhóm value-flag vì cùng lý do.; nay hai policy
// khớp nhau.)
// Bun preloads modules with `--preload`/`-r` exactly like Node does with
// `--require`, so both lists must carry it.
// (Bun preload module bằng `--preload`/`-r` như Node với `--require`.)
const BUN_DANGEROUS_FLAGS: &[&str] = &[
    "--eval",
    "-e",
    "--print",
    "-p",
    "--preload",
    "-r",
    "--require",
];
const DENO_DANGEROUS_FLAGS: &[&str] = &[
    "--eval",
    "-e",
    "--allow-all",
    "-A",
    "--import-map",
    "--config",
    "--cert",
    "--node-modules-dir",
    "--seed",
];
// Node and its TS loader (`tsx`) accept the same flag surface.
const NODE_DANGEROUS_FLAGS: &[&str] = &[
    // inline source
    "--eval",
    "-e",
    "--print",
    "-p",
    // module loading
    "--import",
    "-r",
    "--require",
    "--loader",
    "--experimental-loader",
    "--experimental-policy",
    "--experimental-permission",
    "--permission",
];
const DENO_DANGEROUS_PERMISSIONS: &[&str] = &[
    "--allow-read",
    "--allow-write",
    "--allow-net",
    "--allow-run",
    "--allow-ffi",
    "--allow-hrtime",
];
const NO_DANGEROUS_PERMISSIONS: &[&str] = &[];

pub(in crate::commands::launcher_policy) fn dangerous_flags(
    runtime: Runtime,
) -> &'static [&'static str] {
    match runtime {
        Runtime::Bun => BUN_DANGEROUS_FLAGS,
        Runtime::Deno => DENO_DANGEROUS_FLAGS,
        Runtime::Node | Runtime::Tsx => NODE_DANGEROUS_FLAGS,
    }
}

pub(in crate::commands::launcher_policy) fn dangerous_permissions(
    runtime: Runtime,
) -> &'static [&'static str] {
    match runtime {
        Runtime::Deno => DENO_DANGEROUS_PERMISSIONS,
        Runtime::Bun | Runtime::Node | Runtime::Tsx => NO_DANGEROUS_PERMISSIONS,
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
