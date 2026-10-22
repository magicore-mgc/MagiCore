# Native lib engine facade

Stable public route for `NativeResolution`, `resolve_with_protocol` and `resolve_with_protocol_including_dev`. `mod.rs` exposes the API; `engine.rs` wires six private owners for roots, indexes, closure, graph/lock projection and integrity. Runtime and dev behavior is documented in those owners.

Facade công khai ổn định cho `NativeResolution`, `resolve_with_protocol` và `resolve_with_protocol_including_dev`. `mod.rs` xuất API; `engine.rs` nối sáu owner nội bộ cho root, index, closure, projection graph/lock và integrity. Hành vi runtime/dev được ghi trong từng owner.

The nearest facade test verifies these symbols remain importable through `native::engine`. Runtime status: NOT_RUN.

Test gần facade kiểm tra các symbol tiếp tục import được qua `native::engine`. Runtime status: NOT_RUN.
