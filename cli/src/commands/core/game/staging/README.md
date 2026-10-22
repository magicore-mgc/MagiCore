# Game optimizer staging / Staging optimizer Game

`staging.rs` owns the temporary `.magicore/tmp-optimizer` directory used while preparing the Bevy optimizer. It refuses linked/reparse parent and target paths, reports cleanup failures, and removes `.magicore` only when this operation created it and it is still empty. The shared link guard remains the single safety implementation.

`staging.rs` quản lý thư mục tạm `.magicore/tmp-optimizer` khi chuẩn bị optimizer Bevy. Module từ chối parent/target dạng link hoặc reparse, báo lỗi cleanup và chỉ xóa `.magicore` nếu chính thao tác tạo thư mục đó và nó vẫn rỗng. Bộ kiểm tra link dùng chung vẫn là implementation duy nhất.

The owner-local source tests are in `test/`; no test, build, or formatter has been run as part of the source-first pass.

Test source gần owner nằm trong `test/`; chưa chạy test, build hoặc formatter trong giai đoạn refactor source-first.
