# Marker I/O owner / Owner I/O marker

Owns the `.magicore-package-root.json` path, JSON read/write through `atomic_write`, fast schema/name/version/integrity/tarball checks, and full-content-presence predicate. The `ExtractedPackageMarker` data type remains owned by `crate::cache` in this slice.

Sở hữu path `.magicore-package-root.json`, đọc/ghi JSON qua `atomic_write`, kiểm tra nhanh schema/name/version/integrity/tarball và predicate marker có nội dung đầy đủ. Type `ExtractedPackageMarker` vẫn do `crate::cache` sở hữu trong slice này.

Tests cover persistence round-trip, fast identity, and full-signature presence. Existing `exists` and symlink-follow behavior is preserved and documented as an open security gap; do not report this structural move as a security fix.

Test bao phủ round-trip lưu marker, identity nhanh và chữ ký đầy đủ. Hành vi `exists`/đi theo symlink hiện tại được giữ và ghi nhận là gap security; không báo lần chuyển cấu trúc này là security fix.
