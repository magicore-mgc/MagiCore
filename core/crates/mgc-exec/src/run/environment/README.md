# Guarded execution environment

EN: This owner connects the typed environment error boundary, ShadowPath state, deny-shim files, guarded PATH, and Windows executable identity. `environment.rs` wires the subowners; the stable outer `mod.rs` preserves paths used by process spawn, process monitoring, and Windows verification.

VI: Owner này nối error boundary môi trường, trạng thái ShadowPath, file deny-shim, PATH guard và định danh executable Windows. `environment.rs` wiring các owner con; `mod.rs` bên ngoài giữ nguyên đường dẫn được spawn, process monitor và Windows verification sử dụng.

## Boundaries and guarantees

`shadow_path/` owns temporary-directory lifecycle and refuses to follow substituted directory symlinks on Unix. `guarded_path/` prepends the deny directory and respects the host's PATH key casing. `windows_executable_identity/` keeps the existing cached, fixed-buffer SHA-256 flow and is Windows-only. The shared `EnvironmentError` remains the typed boundary so cleanup and setup failures retain their source chain. Existing tests stay under `test/` for the source-first phase.

`shadow_path/` sở hữu vòng đời thư mục tạm và trên Unix không đi theo symlink thay thế. `guarded_path/` đưa thư mục deny lên đầu và theo đúng quy tắc viết hoa PATH của hệ điều hành. `windows_executable_identity/` giữ luồng SHA-256 cache/buffer cố định hiện tại và chỉ dành cho Windows. `EnvironmentError` dùng chung tiếp tục là boundary typed để giữ source chain của lỗi setup/cleanup. Test hiện có giữ trong `test/` ở giai đoạn refactor source.
