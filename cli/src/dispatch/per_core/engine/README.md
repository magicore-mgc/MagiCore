# Per-core engine command adapter

This owner converts the 14 `EngineCommands` variants into the existing `CommonCommand` dispatch values. It preserves runtime compatibility and delegated-runner flags, command arguments, and nested tool/cache options. The adapter only transports requests; actual behavior and per-core capability checks remain in downstream handlers.

The mapper is exhaustive and performs no fallible work, so it owns no local error type. Nearest behavior tests are grouped under `test/` by runtime, lifecycle, tooling, and cache responsibility.

## Vietnamese

Owner này chuyển 14 variant `EngineCommands` thành giá trị dispatch `CommonCommand` hiện tại. Nó giữ cờ runtime tương thích, runner ủy quyền, đối số lệnh và tùy chọn tool/cache lồng nhau. Adapter chỉ chuyển request; hành vi thật và kiểm tra capability từng core vẫn thuộc handler phía sau.

Mapper exhaustive và không có thao tác phát sinh lỗi nên không cần error type cục bộ. Test gần owner được chia dưới `test/` theo trách nhiệm runtime, lifecycle, tooling và cache.
