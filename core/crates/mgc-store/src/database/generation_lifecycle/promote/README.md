# CAS generation promotion

Owns the atomic transaction boundary for promoting one staging token. `promote/promote.rs` coordinates begin/commit/rollback; `promote_impl/` owns the verify-retire-flip body.

## Promote CAS generation

Quản lý transaction nguyên tử để promote một staging token. `promote/promote.rs` điều phối begin/commit/rollback; `promote_impl/` giữ phần verify-retire-flip.

Keep token validation, SQL order, lock/transaction scope, affected-row checks, typed errors, and diagnostics unchanged. Existing lifecycle tests stay at their registered crate paths until the repository-wide test organization phase.

Giữ nguyên kiểm tra token, thứ tự SQL, phạm vi khóa/transaction, kiểm tra số row, lỗi có type và thông báo. Test lifecycle hiện tại giữ Cargo path đã đăng ký đến phase tổ chức test toàn repository.
