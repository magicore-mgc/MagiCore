//! Runtime identity and ordered dangerous-argument profiles.
//! Định danh runtime và profile argument nguy hiểm có thứ tự.

/// Runtime types whose launcher arguments are checked by this policy.
/// Các runtime có arguments launcher được policy kiểm tra.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Runtime {
    Bun,
    Deno,
    Node,
}

impl Runtime {
    pub fn name(&self) -> &'static str {
        match self {
            Runtime::Bun => "bun",
            Runtime::Deno => "deno",
            Runtime::Node => "node",
        }
    }
}

const BUN_DANGEROUS_FLAGS: &[&str] = &["--eval", "-e", "--print", "-p"];
const DENO_DANGEROUS_FLAGS: &[&str] = &["--eval", "-e", "--allow-all", "-A"];
const NODE_DANGEROUS_FLAGS: &[&str] = &["--eval", "-e", "--print", "-p"];
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
        Runtime::Node => NODE_DANGEROUS_FLAGS,
    }
}

pub(in crate::commands::launcher_policy) fn dangerous_permissions(
    runtime: Runtime,
) -> &'static [&'static str] {
    match runtime {
        Runtime::Deno => DENO_DANGEROUS_PERMISSIONS,
        Runtime::Bun | Runtime::Node => NO_DANGEROUS_PERMISSIONS,
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
