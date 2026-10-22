# MCP tools/list response

Owns the existing advertised MagiCore tool schemas and argument definitions.

## Phạm vi

Folder này sở hữu schema và định nghĩa đối số của các tool MagiCore được công bố. `tools_list.rs` chứa implementation; `mod.rs` chỉ khai báo và export module.

The owner preserves the existing JSON-RPC wire contract and uses shared anyhow/JSON error handling.
