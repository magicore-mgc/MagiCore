# Web bundler owners

This tree owns Web build bundling, asset/workspace preparation, dependency pre-bundling, the native ESM development server, and HMR behavior. `dependency_cache/` validates package names, resolves installed entries, invokes esbuild, and keeps per-server in-memory results; `dev_server/` owns Web HTTP routes, request checks, transpilation, cache use and listener lifecycle; `hmr/` owns change watching, event/version management, WebSocket messages and the embedded browser client. The CLI keeps compatibility exports and process-level Web command supervision.

Cây này sở hữu build bundler Web, chuẩn bị workspace/asset, pre-bundling dependency, native ESM dev server và behavior HMR. `dependency_cache/` kiểm tra tên package, tìm entry đã cài, gọi esbuild và giữ kết quả in-memory theo server; `dev_server/` sở hữu HTTP route Web, request check, transpile, sử dụng cache và vòng đời listener; `hmr/` sở hữu theo dõi thay đổi, quản lý event/version, WebSocket message và browser client nhúng. CLI giữ export tương thích và điều phối command Web ở cấp process.

The private build mirror is temporary and scoped: its guard removes the directory when the build operation returns or errors. Dependency results remain in-memory for the server lifetime; this tree does not claim a global/disk cache or eviction policy.

Build mirror private chỉ tồn tại tạm theo scope: guard xóa thư mục khi thao tác build trả về hoặc lỗi. Kết quả dependency ở memory trong vòng đời server; cây này không tuyên bố có cache global/disk hoặc eviction policy.
