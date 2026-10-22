# AI registry cache roots

This owner maps each registry to its established cache root under the MagiCore data directory. `Registry::Local` remains an explicit caller-selected path. The module performs path selection only; it does not create directories or access the filesystem.

Owner này map mỗi registry tới cache root hiện có dưới thư mục data MagiCore. `Registry::Local` vẫn là path do caller chọn rõ. Module chỉ chọn path; không tạo thư mục và không truy cập filesystem.

Nearest mapping cases are in `test/cache_dir.rs`.

Test mapping gần owner nằm tại `test/cache_dir.rs`.
