# Scaffold template resolution

## Purpose

Coordinates root selection, Web path/layer resolution, monorepo composition, readiness and non-Web core template paths. The processor-facing `Scaffolder` method paths remain stable. Shared scaffold errors stay in `cli/src/scaffold/error.rs`; this owner does not define a duplicate error type.

## Phạm vi

Điều phối chọn root, phân giải path/layer Web, ghép monorepo, readiness và path template của core không phải Web. Giữ nguyên các method path mà processor sử dụng. Lỗi scaffold dùng chung tiếp tục ở `cli/src/scaffold/error.rs`; owner này không tạo error type trùng lặp.
