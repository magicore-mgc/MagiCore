# MCP `mgc_outdated` tool handler

Owns the outdated-dependency query and response mapping for `mgc_outdated`.

## Phạm vi

Folder này sở hữu truy vấn dependency lỗi thời và ánh xạ response cho `mgc_outdated`. `outdated.rs` chứa implementation; `mod.rs` chỉ khai báo và export module.

The owner preserves the existing JSON-RPC wire contract and uses shared anyhow/JSON error handling.
