# Web development server owners

This Web adapter service owns the native ESM development server, HTTP routes, request security, source transpilation, dependency responses, HMR integration and listener lifetime. The CLI keeps only the old import path and process-level lifecycle coordination.

Service adapter Web này sở hữu dev server ESM native, HTTP route, bảo vệ request, transpile source, response dependency, tích hợp HMR và vòng đời listener. CLI chỉ giữ import path cũ và điều phối lifecycle ở cấp process.

`mod.rs` is the public wiring/export point. The sibling `dev_server.rs` wires the named route/compiler/cache owner folders directly; there is no generic intermediary folder. Tests live beside the behavior they protect; the real-server security and HMR integration tests remain in the CLI test target through the compatibility facade.

`mod.rs` là điểm wiring/export public; `dev_server.rs` wiring trực tiếp các folder owner route/compiler/cache ngang hàng, không có folder trung gian chung. Test nằm cạnh behavior được bảo vệ; integration test security/HMR chạy server thật tiếp tục ở test target CLI thông qua facade tương thích.

## Internal owners / Owner nội bộ

Owner folders are direct children: `types/`, `server/`, `imports/`, `html_route/`, `dependency_route/`, `request_path/`, `source_route/`, `source_file_resolution/`, `compile_options/`, `tsconfig/`, `compiled_cache/`, `compiler/` and `transpile/`.

Các folder owner là thư mục con trực tiếp: `types/`, `server/`, `imports/`, `html_route/`, `dependency_route/`, `request_path/`, `source_route/`, `source_file_resolution/`, `compile_options/`, `tsconfig/`, `compiled_cache/`, `compiler/` và `transpile/`.

# Native Web dev server / Dev server Web native

Implements the in-process MgDevServer used by Web development commands. It serves the project index, HMR client/socket, validated dependency bundles, static assets and transpiled source modules.

Triển khai MgDevServer chạy trong process cho lệnh phát triển Web. Server phục vụ index project, HMR client/socket, dependency bundle đã kiểm tra, asset tĩnh và source module đã transpile.

The implementation tree separates public config/state, route handlers, request-path security, import rewriting, compiler options, tsconfig failure handling, compiled-cache I/O, esbuild execution and request orchestration. The named outer `dev_server.rs` wires these owners; the public `mod.rs` exports the adapter API.

Cây triển khai tách cấu hình/state public, handler route, bảo vệ request path, rewrite import, compiler options, lỗi tsconfig, I/O compiled cache, chạy esbuild và điều phối request. File `dev_server.rs` có tên rõ wiring các owner; `mod.rs` public export API adapter.

The implementation contract is HTTP and compilation behavior: preserve route order, response status/body/headers, fail-closed path checks, cache identity/context, raw-cache semantics, HMR watcher lifetime and English diagnostics. Unit tests are registered beside compile options, request-path and import-string owners; route and real-server tests remain beside their existing owners/CLI integration target.

Contract triển khai là hành vi HTTP và compile: giữ thứ tự route, status/body/header, chặn path fail-closed, identity/ngữ cảnh cache, semantics cache raw, vòng đời watcher HMR và diagnostic tiếng Anh. Unit test được đăng ký cạnh owner compile options, request-path, import-string; route test và real-server integration vẫn cạnh owner/call path CLI hiện có.
