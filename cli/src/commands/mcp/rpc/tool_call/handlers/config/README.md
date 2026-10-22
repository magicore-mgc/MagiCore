# MCP `mgc_config` tool handler

Owns the redacted local-configuration query and response mapping for `mgc_config`.

## Phạm vi

Folder này sở hữu việc truy vấn config local đã che dữ liệu nhạy cảm và ánh xạ response cho `mgc_config`. `config.rs` chứa implementation; `mod.rs` chỉ khai báo và export module.

The owner preserves the existing JSON-RPC wire contract and uses shared anyhow/JSON error handling.
