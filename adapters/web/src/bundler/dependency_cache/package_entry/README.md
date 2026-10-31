# Dependency package entry resolution

This owner resolves the installed package directory first, then selects its entry. The package directory may be a pnpm/workspace symlink; the resolved directory becomes its root.

Request handling follows Node's resolution order:

1. A bare package request reads `exports["."]` (or a condition set used as `exports`), then falls back to browser/module/main and `index.js`.
2. A requested subpath that is an existing file is returned directly.
3. A requested subpath that is an existing directory reads its `exports` subpath (if declared) and then that directory's own manifest/index.
4. A subpath with nothing on disk is tried against the manifest `exports` subpath map (exact key, dotted `./x.js` key, then the most specific `./prefix/*` pattern), then the literal file, then `<subpath>.js`, then `<subpath>/index.js` while the directory exists.

Only after every candidate is absent is the subpath reported missing, which surfaces as a 404 on the deps route. Manifest entry paths, `exports` targets and fallback files must canonicalize to regular files under the package root. Relative `./` and in-package `../` forms remain valid; absolute, cross-platform separator/device and outside-target paths are rejected.

Request prefixes are built by string concatenation, never by a `format!` that merges the literal `.` with the literal `./`: current toolchains fold that literal pair into `..`.

Owner này resolve thư mục package đã cài trước, rồi chọn entry. Thư mục package có thể là symlink pnpm/workspace; thư mục sau resolve là root.

Request đi theo đúng thứ tự Node:

1. Request package trần đọc `exports["."]` (hoặc bộ condition dùng làm `exports`), rồi mới fallback browser/module/main và `index.js`.
2. Subpath là file có thật được trả thẳng.
3. Subpath là thư mục có thật đọc `exports` subpath (nếu khai báo) rồi manifest/index của chính thư mục đó.
4. Subpath không có gì trên đĩa được thử lần lượt: map `exports` subpath (key chính xác, key nối đuôi `./x.js`, rồi pattern `./prefix/*` cụ thể nhất), file nguyên văn, `<subpath>.js`, `<subpath>/index.js` khi thư mục tồn tại.

Chỉ khi hết candidate mới báo miss — tương ứng 404 trên route deps. Entry manifest, target `exports` và fallback phải canonicalize thành regular file trong package root. Giữ dạng relative `./` và `../` nằm trong package; chặn absolute, separator/device liên OS và target ra ngoài.

Tiền tố request dựng bằng nối chuỗi, không dùng `format!` ghép literal `.` với `./`: toolchain hiện tại gộp cặp literal đó thành `..`.
