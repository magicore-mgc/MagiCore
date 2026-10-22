# JSON-RPC method-not-found response

Owns the existing JSON-RPC method-not-found error response.

## Phạm vi

Folder này sở hữu response lỗi method-not-found hiện có của JSON-RPC. `method_not_found.rs` chứa implementation; `mod.rs` chỉ khai báo và export module.

The owner preserves the existing JSON-RPC wire contract and uses shared anyhow/JSON error handling.
