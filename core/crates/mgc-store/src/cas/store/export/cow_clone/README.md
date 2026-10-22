# Cow Clone owner

The descriptor COW owner uses macOS `fclonefileat` and Linux `FICLONE`; unsupported targets/filesystems return to the caller for plain-copy fallback. Its path wrapper exists only for the staging-kind probe and opens a no-follow regular file first.

Owner COW theo descriptor dùng macOS `fclonefileat` và Linux `FICLONE`; target/filesystem chưa hỗ trợ trả quyền cho caller copy thường. Wrapper nhận path chỉ phục vụ staging-kind probe và mở file thường không theo symlink trước.

Nearest source test: `test/cow_clone.rs`; it exercises this owner implementation directly. Runtime status: NOT_RUN.
