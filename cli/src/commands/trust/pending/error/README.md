# Trust pending errors

`error.rs` owns typed failures for lock acquisition, pending-journal recovery, policy parsing, package-tree inspection, malformed manifests, and oversized manifests. The variants define the recovery class; source errors are retained where they exist, and the existing English diagnostic text stays stable.

# Lỗi trust pending

`error.rs` sở hữu lỗi typed cho acquire lock, pending-journal recovery, parse policy, quét cây package, manifest sai và manifest quá lớn. Variant phân loại hướng khôi phục; source gốc được giữ nếu có và thông báo tiếng Anh hiện tại được bảo toàn.
