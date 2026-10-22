# Cache prune removal owner

This owner rechecks each selected path with non-following metadata, removes only regular files or directories, reports logical bytes from regular files, and removes a regular `.meta.json` sidecar after the cache entry. It rejects a model path that has become a symlink and leaves symlink targets untouched.

Owner này kiểm tra lại path bằng metadata không đi theo liên kết, chỉ xóa file thường hoặc thư mục, tính logical bytes từ file thường, rồi xóa sidecar `.meta.json` thường. Nếu model path biến thành symlink, owner từ chối và không động vào đích liên kết.

`remove_dir_all` itself does not follow symbolic links according to the Rust standard-library contract; platform-specific TOCTOU limitations remain and require verification on supported OS lanes. See [Rust `remove_dir_all`](https://doc.rust-lang.org/std/fs/fn.remove_dir_all.html). Nearest cases are in `test/removal.rs`.

Theo hợp đồng Rust standard library, `remove_dir_all` không đi theo symlink; giới hạn TOCTOU theo nền tảng vẫn cần xác minh trên các OS được hỗ trợ. Xem [Rust `remove_dir_all`](https://doc.rust-lang.org/std/fs/fn.remove_dir_all.html). Test gần owner nằm tại `test/removal.rs`.
