# Candidate Plan

## Responsibility

Resolves each unreferenced database package to the Web virtual-store directory, validates the candidate beneath the canonical root, and computes checked logical payload totals before deletion.

## Trách nhiệm

Ánh xạ package không tham chiếu trong DB sang thư mục Web virtual-store, xác thực candidate nằm dưới root canonical và tính tổng payload logic có kiểm tra tràn trước khi xóa.

Scoped package names use the same `@scope+name@version` directory as the Web materializer. `error.rs` owns candidate path and measurement failures. Missing directories produce zero projected bytes; unsafe candidates abort the entire plan. `test/` covers naming, missing data and symlink refusal. This owner performs no filesystem deletion.

Package có scope dùng cùng tên thư mục `@scope+name@version` với Web materializer. `error.rs` sở hữu lỗi path candidate và phép đo. Thư mục thiếu được tính 0 byte dự kiến; candidate không an toàn làm hủy toàn bộ plan. `test/` kiểm tra tên, dữ liệu thiếu và từ chối symlink. Owner này không tự xóa filesystem.
