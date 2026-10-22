# MCP `mgc_workspace_info` tool handler

Owns the workspace topology query and response mapping for `mgc_workspace_info`.

## Phạm vi

Folder này sở hữu truy vấn topology workspace và ánh xạ response cho `mgc_workspace_info`. `workspace_info.rs` chứa implementation; `mod.rs` chỉ khai báo và export module.

The owner preserves the existing JSON-RPC wire contract and uses shared anyhow/JSON error handling.
