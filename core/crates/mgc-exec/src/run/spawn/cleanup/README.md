# Spawn cleanup-result owner

This owner combines a completed execution result with failures from guarded PATH cleanup and deferred Windows handle cleanup. It preserves the primary execution error as the source when execution and cleanup both fail; a cleanup-only failure remains visible as the returned error.

Owner này ghép kết quả thực thi với lỗi dọn guarded PATH và deferred Windows handle. Khi cả execution lẫn cleanup cùng lỗi, lỗi execution vẫn là source chính; lỗi cleanup đơn lẻ vẫn được trả về.

`cleanup.rs` owns both result pairings; `mod.rs` exports them only to the spawn implementation. `test/` covers the Windows-handle and guarded-environment failure pairings. `docs/` tracks the owner checklist and source/runtime status. Tests are source-only and NOT_RUN pending the global runtime gate.

`cleanup.rs` sở hữu hai nhánh ghép kết quả; `mod.rs` chỉ export chúng cho spawn implementation. `test/` kiểm tra hai nhánh cleanup Windows handle và guarded environment. `docs/` theo dõi checklist và trạng thái source/runtime của owner. Test mới chỉ có trong source, chưa chạy do runtime gate toàn repo.
