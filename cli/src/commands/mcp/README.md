# Native MCP server

Owns the CLI's built-in JSON-RPC 2.0 stdio server. `mod.rs` is wiring only; the same-name `mcp.rs` command coordinator preserves `commands::mcp::run`. Stdio ownership/capture, protocol methods, resources, tool definitions and each tool handler live in focused owners.

## Phạm vi

Folder này sở hữu server JSON-RPC 2.0 qua stdio tích hợp trong CLI. `mod.rs` chỉ wiring; coordinator cùng tên `mcp.rs` giữ nguyên `commands::mcp::run`. Quản lý stdio/capture, method protocol, resource, khai báo tool và từng handler nằm trong owner riêng.

The existing protocol version, output-channel separation, capture cap, response shapes, tool/resource schemas and delegated command behavior are frozen for this source-only refactor. Runtime correctness remains unverified until the global test phase.
