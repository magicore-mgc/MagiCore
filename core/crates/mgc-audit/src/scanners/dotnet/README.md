# .NET NuGet scanner

Builds NuGet audit pins from `packages.lock.json` files and sends them through the shared OSV client. The scanner reads MGC-owned project files; it does not spawn `dotnet` or run restore.

Dựng pin audit NuGet từ `packages.lock.json` và gửi qua OSV client dùng chung. Scanner đọc file do MagiCore quản lý; không chạy `dotnet` hoặc restore.

`mod.rs` preserves the scanner facade. `dotnet.rs` owns audit/report orchestration; `lockfile/`, `solution/`, `project_path/`, `pin_collection/`, and `error/` own parsing and safety boundaries. Existing `tests/lane_dotnet_test.rs` remains the product-route integration suite.
