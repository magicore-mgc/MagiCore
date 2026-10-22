# Unix profile entry update

Owns bounded profile reads, symlink/regular-file checks, managed-block replacement and delegation to atomic persistence. Existing `anyhow` error propagation is preserved. Public API cases remain in `core/crates/mgc-platform/tests/paths.rs`.

## Vietnamese

Owner này giới hạn dung lượng đọc profile, từ chối symlink/file không thường, thay block được quản lý và gọi owner ghi nguyên tử. Giữ cách truyền lỗi `anyhow`; ca API public vẫn ở `core/crates/mgc-platform/tests/paths.rs`.
