# MCP `mgc_audit` tool handler

Owns the `fix` argument, audit command invocation, and response mapping for `mgc_audit`.

## Phạm vi

Folder này sở hữu argument `fix`, việc gọi lệnh audit và ánh xạ response cho `mgc_audit`. `audit.rs` chứa implementation; `mod.rs` chỉ khai báo và export module.

The owner preserves the existing JSON-RPC wire contract and uses shared anyhow/JSON error handling.
