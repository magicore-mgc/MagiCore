# Shared publish contracts

This namespace exports two data-only contracts: `PublishOptions` for request configuration and `PublishSummary` for the serialized result. It does not perform registry I/O, packaging, authentication or publishing.

Namespace này export hai contract chỉ chứa dữ liệu: `PublishOptions` cho cấu hình request và `PublishSummary` cho kết quả serialize. Nó không thực hiện I/O registry, đóng gói, xác thực hay publish.

These contracts are shared across `mgc-publish`, `mgc-pack` and the CLI; the original source note records `01 §5, Phase 0`.

Các contract này được dùng chung giữa `mgc-publish`, `mgc-pack` và CLI; ghi chú source cũ tham chiếu `01 §5, Phase 0`.

The `mod.rs` file only wires and re-exports the two owners. Each owner keeps its own source and nearest tests.

File `mod.rs` chỉ nối module và re-export hai owner. Mỗi owner giữ source và test gần nhất của mình.
