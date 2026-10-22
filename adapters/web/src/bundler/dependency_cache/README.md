# Web dependency cache and bundler

This owner validates package names, resolves local package entry files, bundles them for browser ESM, and caches the result for one development-server lifetime. It does not own the HTTP response or route status.

Owner này kiểm tra tên package, tìm entry file local, bundle thành browser ESM và cache kết quả trong một vòng đời dev server. Owner không quản lý HTTP response hoặc route status.

`DepsCache` retains the existing in-memory, session-scoped `Arc<RwLock<HashMap<...>>>` policy. It has no disk/global persistence or eviction policy. Invalid names are rejected at this public adapter boundary before a path is built; the CLI route uses the same validator to preserve its 403 response. Before successful output is cached, the esbuild metafile is checked against canonical installed-package roots. This prevents reported out-of-root inputs from being returned, but happens after esbuild has read them and is not a filesystem sandbox.

`DepsCache` giữ chính sách in-memory theo session `Arc<RwLock<HashMap<...>>>` hiện có. Owner không có persistence disk/global hay eviction. Tên không hợp lệ bị chặn tại boundary public của adapter trước khi tạo path; CLI route dùng cùng validator để giữ response 403. Trước khi cache output thành công, metafile esbuild được kiểm với root package đã cài canonical. Điều này chặn input ngoài root được báo khỏi response nhưng diễn ra sau khi esbuild đã đọc file, không phải filesystem sandbox.

Package entry resolution canonicalizes the package directory, preserves intentional pnpm/workspace directory symlinks, supports plain/scoped subpaths as direct files or directories, and confines selected manifest/fallback entries to that canonical package root with lexical checks before lookup. After esbuild finishes, the metafile gate rejects any reported input outside installed package roots before output enters the cache or HTTP response. This runs after esbuild reads those files and is not a filesystem sandbox; TOCTOU and hard-link issues remain open.

Source is decomposed into named owners for cache orchestration, package entry resolution/errors, esbuild execution, public result data, validation, and bundling errors. Tests sit beside the behavior each file owns.

Package entry được canonicalize theo thư mục package; vẫn cho symlink thư mục pnpm/workspace có chủ đích, hỗ trợ subpath thường/scoped dạng file trực tiếp hoặc directory, và giới hạn manifest/fallback trong canonical package root bằng lexical check trước filesystem lookup. Sau khi esbuild chạy xong, metafile gate từ chối input ngoài root package đã cài trước khi output vào cache hoặc HTTP response. Gate chạy sau khi esbuild đọc file nên không phải filesystem sandbox; TOCTOU và hard-link còn mở.

Source được tách thành owner có tên cho điều phối cache, tìm package entry/error, chạy esbuild, data kết quả public, validation và lỗi bundling. Test đặt cạnh behavior của từng file.
