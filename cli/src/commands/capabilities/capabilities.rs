//! Capability command coordination — điều phối lệnh capability.
//! Builds one manifest for both CLI output and MCP resource reads.
//! Tạo một manifest dùng chung cho CLI và MCP.

use anyhow::Result;

use super::core_catalog::{ALL_CORES, capabilities_for_core};
use super::dependency_ownership::dependency_ownership;

/// `mgc capabilities [--core <core>]` — with `--core`, prints the single
/// chotted shape; without it, prints all built cores (the lifecycle matrix
/// consumes the all-cores shape).
/// `mgc capabilities [--core <core>]` — có `--core`, in shape đơn đã chốt;
/// không có, in mọi core đã build (lifecycle matrix dùng shape all-cores).
pub fn run(core: Option<&str>) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(&json_payload(core)?)?);
    Ok(())
}

/// Build the capability manifest as structured JSON without writing to stdout.
/// Tạo capability manifest dạng JSON mà không ghi ra stdout.
pub fn json_payload(core: Option<&str>) -> Result<serde_json::Value> {
    match core {
        Some(name) => {
            let caps = capabilities_for_core(name)?;
            Ok(serde_json::json!({
                "version": env!("CARGO_PKG_VERSION"),
                "core": name,
                "capabilities": caps,
                "dependency_ownership": dependency_ownership(name),
                "framework_qualification": crate::commands::framework_records::qualification_json(name),
                "everything_else": "unsupported",
            }))
        }
        None => {
            let mut cores = Vec::new();
            for name in ALL_CORES {
                // Cores not compiled into this binary are skipped — the
                // matrix gate treats an absent row as fail-closed.
                // Core không được compile vào binary bị bỏ qua — gate matrix
                // coi hàng vắng mặt là fail-closed.
                let caps = match capabilities_for_core(name) {
                    Ok(caps) => caps,
                    Err(_) => continue,
                };
                cores.push(serde_json::json!({
                    "core": name,
                    "capabilities": caps,
                    "dependency_ownership": dependency_ownership(name),
                    "framework_qualification": crate::commands::framework_records::qualification_json(name),
                    "everything_else": "unsupported",
                }));
            }
            let payload = serde_json::json!({
                "version": env!("CARGO_PKG_VERSION"),
                "cores": cores,
            });
            Ok(payload)
        }
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
