# Web install `.bin` / `.bin` của Web install

This owner rebuilds package executable links after installation. Its public API remains at `crate::install::bin`; it delegates directory cleanup, installed-manifest parsing, package target checks, OS link creation, executable permissions and local errors to named owners.

Owner này dựng lại link executable package sau cài đặt. API public vẫn ở `crate::install::bin`; xử lý cleanup, đọc manifest đã cài, kiểm tra target package, tạo link theo OS, quyền executable và lỗi cục bộ được giao cho các owner riêng.

The rebuild is called by install finalization. Strict cleanup only prunes stale MGC virtual-store links and dangling links; package targets are rejected when they contain `..` or canonicalize outside the package directory. Unix uses symlinks and mode bits; non-Unix writes the existing Node `.cmd` shim and reports no executable bit.

Finalize install gọi hàm rebuild. Cleanup strict chỉ dọn link virtual-store MGC đã cũ và link dangling; target bị bỏ qua nếu có `..` hoặc canonical path thoát khỏi package. Unix dùng symlink và mode bit; non-Unix ghi shim Node `.cmd` hiện tại và không báo executable bit.

Errors are converted back to their original `MgError` variant so existing callers retain their category and display text. No direct `.bin` unit test was found at S397 baseline; behavior tests and Unix/Windows runtime checks remain pending in the repository-wide test phase. Do not infer runtime support from this source move.

Lỗi được chuyển lại `MgError` gốc để caller giữ nguyên loại và nội dung hiển thị. Baseline S397 chưa tìm thấy unit test `.bin`; test hành vi và kiểm tra runtime Unix/Windows còn chờ phase test toàn repo. Không suy ra hỗ trợ runtime chỉ từ việc chuyển source.
