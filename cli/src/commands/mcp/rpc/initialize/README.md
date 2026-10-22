# MCP initialize response

Owns the `initialize` JSON-RPC response, including the advertised protocol version, capabilities, and server identity.

## Phạm vi

Folder này sở hữu response `initialize`, gồm phiên bản giao thức, capabilities và danh tính server được công bố. `initialize.rs` chứa implementation; `mod.rs` chỉ khai báo và export module.

The owner preserves the existing JSON-RPC wire contract and uses shared anyhow/JSON error handling.
