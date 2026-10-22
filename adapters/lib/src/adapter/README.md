# Lib adapter owners

`mod.rs` wires the capability owners and preserves the `LibAdapter` exports. `native_registry/` is the shared protocol-construction boundary for Lib's Rust, Python, Go, Maven, and NuGet lanes. `dependency_resolver/` owns the resolver trait route; `resolve_first/` prepares one native package add without writing files. TypeScript remains delegated to the Web adapter.

Each owner keeps its behavior, typed local errors where needed, nearest Rust test modules, and bilingual README together. Existing adapter integration/native-engine tests remain under `adapters/lib/tests/`.

## Phạm vi

`mod.rs` nối các owner capability và giữ export `LibAdapter`. `native_registry/` là ranh giới dựng protocol dùng chung cho các lane Rust, Python, Go, Maven và NuGet của Lib. `dependency_resolver/` sở hữu route trait resolver; `resolve_first/` chuẩn bị add một package native mà chưa ghi file. TypeScript tiếp tục qua Web adapter.

Mỗi owner giữ behavior, lỗi cục bộ có kiểu khi cần, test Rust gần nhất và README song ngữ cùng vị trí. Integration/native-engine tests hiện có vẫn ở `adapters/lib/tests/`.
