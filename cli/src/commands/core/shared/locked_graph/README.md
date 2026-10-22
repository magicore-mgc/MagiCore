# locked_graph

The shared core lock-reuse path. `locked_graph.rs` wires loading, manifest-validation and graph-conversion owners while preserving the established `core::shared::*` functions. Owner-scoped closure checks remain fail-closed.

Đường tái sử dụng lock dùng chung giữa các core. `locked_graph.rs` nối owner loading, manifest validation và graph conversion, đồng thời giữ các function `core::shared::*` hiện hữu. Kiểm tra closure theo owner tiếp tục fail-closed.

The current error surface continues through the existing anyhow/CLI renderer boundary; S212 changes module ownership only.
Ranh giới lỗi hiện tại vẫn đi qua anyhow/CLI renderer; S212 chỉ thay đổi ownership module.
