# run command owners

This module preserves the `crate::commands::run` entry path while separating the shared task-run lifecycle by responsibility. `mod.rs` is wiring and exports only; `run.rs` owns the public CLI command entry.

- `run_with_hooks/` coordinates pre/main/post scripts and preserves the current error precedence.
- `script_resolution/` reads `mgc.toml` and `package.json` scripts and keeps the existing source precedence.
- `script_policy/` enforces native-runtime and package-manager restrictions before execution.
- `task_execution/` resolves policy context, constructs the task request and delegates to the guarded executor.
- `task_execution/executor/` assembles environment and `ExecOptions`, then calls `mgc-exec`.
- `runtime_policy/` keeps timeout defaults and project runtime detection used by optimizer environment loading.
- `test/` contains the nine existing cases grouped by resolution, lifecycle and execution policy.

Existing diagnostic constructors and their output/source behavior remain unchanged. No local error module was added because this batch introduces no new error contract. The test suite was moved, not executed; compile/runtime status is `NOT_RUN`.

# run command owners (VI)

Module này giữ nguyên đường vào `crate::commands::run` và tách vòng đời task dùng chung theo trách nhiệm. `mod.rs` chỉ wiring/export; `run.rs` sở hữu entry của CLI.

- `run_with_hooks/` điều phối pre/main/post và giữ nguyên thứ tự ưu tiên lỗi hiện tại.
- `script_resolution/` đọc script từ `mgc.toml` và `package.json`, giữ nguyên thứ tự ưu tiên nguồn.
- `script_policy/` chặn runtime native không hợp lệ và package manager ngoài trước khi chạy.
- `task_execution/` nạp ngữ cảnh policy, tạo task request rồi giao cho executor có rào chắn.
- `task_execution/executor/` dựng environment và `ExecOptions`, sau đó gọi `mgc-exec`.
- `runtime_policy/` giữ timeout mặc định và nhận diện runtime project cho optimizer environment.
- `test/` chứa chín test cũ, chia theo resolution, lifecycle và execution policy.

Các diagnostic constructor hiện tại cùng output/source được giữ nguyên. Batch không thêm error module vì không tạo hợp đồng lỗi mới. Test source đã được chuyển nhưng chưa chạy; compile/runtime là `NOT_RUN`.
