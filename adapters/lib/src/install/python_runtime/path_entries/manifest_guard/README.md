# Python manifest guard

Rejects symlinked or externally owned Python manifests and checks every declared dependency against the active core's locked name and version range.
`error.rs` owns typed manifest policy, filesystem, and lock-pin mismatch failures.

## Rào manifest Python

Từ chối manifest Python là symlink hoặc thuộc package manager khác; đối chiếu tên và dải phiên bản từng dependency với lock của core hiện tại.
`error.rs` sở hữu lỗi typed về chính sách manifest, filesystem và sai khác pin trong lock.
