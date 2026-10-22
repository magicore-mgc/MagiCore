# Package cache concurrency tests / Test đồng thời package cache

These two integration tests cover atomic publication under simultaneous writers and the conflicting-content fail-closed rule. Each behavior folder has a same-name test implementation and a wiring-only `mod.rs`. The parent `mod.rs` includes those owner files into the Cargo target's root so test names, order, and target discovery stay unchanged.

Hai integration test này kiểm tra publish nguyên tử khi nhiều writer chạy đồng thời và quy tắc từ chối nội dung xung đột. Mỗi folder hành vi có implementation test trùng tên và `mod.rs` chỉ wiring. `mod.rs` cha include các owner vào root Cargo target để giữ nguyên tên, thứ tự test và discovery của target.

The test target is intentionally not run in the current source-first phase; runtime verification remains pending.

Target chưa chạy trong phase source-first hiện tại; xác minh runtime vẫn chờ.

Raw metadata dot-segment containment is covered by the separate `package_cache_path_security` target so this concurrency target remains scoped to writer behavior.

Containment của dot-segment metadata thô được kiểm tra ở target riêng `package_cache_path_security`, giữ target concurrency tập trung vào writer behavior.
