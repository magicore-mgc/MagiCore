# Tarball entry import owner

Owns the public tarball-entry coordinator and delegates every entry to the shared verified/atomic writer. It does not add a second persistence path or alter StoreError. Concurrent same-content and corrupted-dedup behavior remain covered by the frozen integration suite.

Phụ trách coordinator public import entry tarball và giao từng entry cho writer chung đã xác minh/nguyên tử. Không tạo đường lưu thứ hai và không đổi StoreError. Hành vi cùng nội dung đồng thời và dedup bị hỏng vẫn nằm trong integration suite đóng băng.

Nearest source regression test: test/import_tarball_entries.rs.
