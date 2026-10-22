# MCP resources/list response

Owns the existing advertised workspace, capability, and redacted-configuration resources.

## Phạm vi

Folder này sở hữu danh sách resource workspace, capability và config đã che bí mật được công bố. `resources_list.rs` chứa implementation; `mod.rs` chỉ khai báo và export module.

The owner preserves the existing JSON-RPC wire contract and uses shared anyhow/JSON error handling.
