# Regular file copy / Copy file thường

This owner opens the source through `mgc-platform`'s final-component no-follow API, streams into a unique same-directory staging file, preserves source permissions, and atomically replaces a regular destination. Its parent must already exist as a real directory. Path-based parent replacement races remain outside this helper's guarantee.

Owner này mở source bằng API no-follow component cuối của `mgc-platform`, stream vào file staging duy nhất cùng thư mục, giữ permission source và thay atomic destination file thường. Thư mục cha phải tồn tại và là directory thật. Race thay parent path vẫn nằm ngoài bảo đảm của helper.

Implementation: `file_copy.rs`; local failures: `error.rs`; tests: `test/`.
