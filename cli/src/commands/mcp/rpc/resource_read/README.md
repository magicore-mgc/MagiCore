# MCP resource read response

Owns resource URI selection and the existing workspace, capabilities, and redacted-configuration response mapping.

## Phạm vi

Folder này sở hữu việc chọn URI resource và ánh xạ response workspace, capabilities, hoặc config local đã che bí mật. `resource_read.rs` chứa implementation; `mod.rs` chỉ khai báo và export module.

The owner preserves the existing JSON-RPC wire contract and uses shared anyhow/JSON error handling.
