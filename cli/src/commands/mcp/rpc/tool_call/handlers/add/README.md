# MCP `mgc_add` tool handler

Owns argument extraction, delegated-runtime opt-in forwarding, command invocation, and response mapping for `mgc_add`.

## Phạm vi

Folder này sở hữu việc đọc arguments, chuyển tiếp opt-in runtime delegated, gọi lệnh và ánh xạ response cho `mgc_add`. `add.rs` chứa implementation; `mod.rs` chỉ khai báo và export module.

The owner preserves the existing JSON-RPC wire contract and uses shared anyhow/JSON error handling.
