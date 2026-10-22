# Package cache / Cache package

This folder owns the package tarball and registry-metadata cache exposed by `mgc-store`. `mod.rs` is only the stable module facade; `cache/cache.rs` contains the public `PackageCache` API. `path/` owns cache path construction and `entry/` owns unique staging files plus atomic publication.

Thư mục này quản lý cache tarball package và metadata registry do `mgc-store` cung cấp. `mod.rs` chỉ là facade ổn định; `cache/cache.rs` chứa API công khai `PackageCache`. `path/` quản lý tạo đường dẫn cache; `entry/` quản lý file staging duy nhất và publish nguyên tử.

The module keeps the existing public routes. Common lowercase package names and ASCII-safe versions that round-trip to the same structured value keep their existing paths; non-portable names and unsafe/non-roundtripping versions use versioned BLAKE3 directories. Version hashes encode every identity field separately, so equal display strings do not alias. This prevents lexical traversal from raw metadata dot segments and malformed prerelease text. Existing scoped/separator and unsafe-version cache entries may become misses and are not automatically migrated. A pre-existing symlink in a cache parent can still redirect filesystem operations and remains open security work.

Module giữ nguyên đường dẫn API. Tên package chữ thường phổ biến và version ASCII an toàn có round-trip đúng cấu trúc giữ path cũ; tên không portable và version lỗi/không round-trip dùng thư mục BLAKE3 có version. Hash version mã hóa riêng từng field identity, nên cấu trúc khác nhau nhưng cùng text hiển thị không alias. Cơ chế này chặn lexical traversal từ metadata dot-segment thô và prerelease lỗi. Cache entry scoped/dùng separator và version không an toàn có thể thành cache miss, không tự migrate. Symlink có sẵn trong cache parent vẫn có thể chuyển hướng filesystem operation và còn là đầu việc bảo mật.

`PackageCache::open_tarball` returns a no-follow regular-file handle for bounded-memory consumers. `get_tarball` remains a compatibility helper that materializes the entire artifact in a `Vec`; install paths should use the file API. The opener rejects a final symlink/reparse point, while parent-directory containment remains a separate open issue.

`PackageCache::open_tarball` trả file handle thường, no-follow cho caller cần đọc với bộ nhớ giới hạn. `get_tarball` vẫn là helper tương thích đọc toàn bộ artifact vào `Vec`; install nên dùng API file. Hàm mở từ chối symlink/reparse ở path cuối, còn containment thư mục cha vẫn là đầu việc riêng.
