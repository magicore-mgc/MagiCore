# MCP `mgc_versions` tool handler

Owns package-name validation, registry version query, and response mapping for `mgc_versions`.

## Phạm vi

Folder này sở hữu xác thực tên package, truy vấn phiên bản registry và ánh xạ response cho `mgc_versions`. `versions.rs` chứa implementation; `mod.rs` chỉ khai báo và export module.

The owner preserves the existing JSON-RPC wire contract and uses shared anyhow/JSON error handling.
