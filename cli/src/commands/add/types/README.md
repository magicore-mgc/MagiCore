# Add request type

`AddRequest` carries the existing `mgc_add` package list and flags from the public coordinator to the lane dispatcher. It is private to the `commands::add` module tree; it does not replace per-core contracts or expose a new CLI/MCP schema.

## Tiếng Việt

`AddRequest` chuyển danh sách package và flag `mgc_add` hiện có từ coordinator công khai sang dispatcher lane. Type chỉ dùng trong cây module `commands::add`; không thay contract per-core và không mở schema CLI/MCP mới.
