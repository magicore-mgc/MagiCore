use crate::allowlist::parse_script_invocation;
use anyhow::Result;

/// Parse a simple single-command script without invoking a shell.
/// Parse script đơn lệnh để tránh shell injection trong lifecycle/task runner.
pub fn parse_simple_script(script: &str) -> Result<(String, Vec<String>)> {
    let invocation = parse_script_invocation(script)?;
    Ok((invocation.program, invocation.args))
}
