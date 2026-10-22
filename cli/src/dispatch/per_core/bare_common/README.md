# Per-core bare common adapter

This owner maps shared bare CLI commands into `CommonCommand`. It deliberately leaves Install, Add, Remove, Update, and List unmatched (`None`); the coordinator then routes them through ecosystem detection and the core-specific bare package-operation dispatcher. Keep this boundary so shared commands cannot swallow package operations.

The adapter only converts values and has no fallible operation, so it owns no local error type. Tests cover all 14 mapped variants and all five intentional fallthrough cases without executing handlers.

## Vietnamese

Owner này ánh xạ lệnh bare CLI dùng chung sang `CommonCommand`. Nó cố ý không nhận Install, Add, Remove, Update và List (`None`); coordinator tiếp tục chuyển các lệnh đó qua nhận diện ecosystem và dispatcher package-operation theo core. Giữ ranh giới này để mapper lệnh chung không nuốt mất thao tác package.

Adapter chỉ chuyển đổi giá trị, không có thao tác phát sinh lỗi nên không cần error type cục bộ. Test bao phủ đủ 14 variant được ánh xạ và năm nhánh fallthrough có chủ ý mà không thực thi handler.
