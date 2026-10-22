# MCP `mgc_install` tool handler

Owns package/frozen argument extraction, install command invocation, and response mapping for `mgc_install`.

## Phạm vi

Folder này sở hữu việc đọc arguments package/frozen, gọi lệnh install và ánh xạ response cho `mgc_install`. `install.rs` chứa implementation; `mod.rs` chỉ khai báo và export module.

The owner preserves the existing JSON-RPC wire contract and uses shared anyhow/JSON error handling.
