# Lib install coordinator

This folder owns the Lib adapter's install lifecycle coordinator. `mod.rs` preserves the existing `crate::install::run_install` route; `install.rs` keeps the call order and language dispatch. It delegates policy and preflight work to the neighboring owners and does not resolve or fetch packages itself.

`offline_policy/` holds the existing offline allowlist. `preflight/` checks lock coverage, Rust Cargo compatibility and the existing lock before native installs. `empty_graph/` preserves the no-network fast path and its lock-file side effects. TypeScript still delegates to the Web adapter; Rust, Python, Go, Maven and NuGet keep their existing native install owners and lock publication behavior.

No coordinator-local error type is introduced: this owner keeps the established `MgError`/`MgResult` boundary and propagates native lane errors. Each behavior owner has its own nearest `test/` folder; tests remain `NOT_RUN` until the repository-wide source refactor closes.

## Điều phối install Lib

Folder này sở hữu coordinator lifecycle install của adapter Lib. `mod.rs` giữ nguyên đường gọi `crate::install::run_install`; `install.rs` giữ thứ tự lời gọi và dispatch ngôn ngữ. File điều phối ủy quyền policy và preflight cho owner gần bên cạnh, không tự resolve hoặc tải package.

`offline_policy/` chứa allowlist offline hiện có. `preflight/` kiểm tra coverage lock, tương thích Cargo cho Rust và lock hiện tại trước native install. `empty_graph/` giữ fast path không mạng và side effect lock vốn có. TypeScript tiếp tục ủy quyền cho Web adapter; Rust, Python, Go, Maven và NuGet giữ native owner cùng chính sách ghi lock hiện tại.

Coordinator không thêm error type riêng: owner giữ ranh giới `MgError`/`MgResult` đang dùng và propagate lỗi lane native. Mỗi behavior owner có folder `test/` gần nhất; test vẫn `NOT_RUN` tới khi refactor source toàn repo đóng.
