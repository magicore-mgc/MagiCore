# MCP `mgc_uninstall` tool handler

Owns package/runtime argument extraction, remove command invocation, and response mapping for `mgc_uninstall`.

## Phạm vi

Folder này sở hữu việc đọc arguments package/runtime, gọi lệnh remove và ánh xạ response cho `mgc_uninstall`. `uninstall.rs` chứa implementation; `mod.rs` chỉ khai báo và export module.

The owner preserves the existing JSON-RPC wire contract and uses shared anyhow/JSON error handling.
