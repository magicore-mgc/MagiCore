# Gradle lock owner / Owner Gradle lock

This owner parses exact `group:artifact:version[:configuration]` pins, collapses repeated configurations, and refuses conflicting versions. `GradleLockError` maps to the existing `MgError::Other` category and preserves its current English text.

Owner này parse pin chính xác `group:artifact:version[:configuration]`, gộp configuration lặp và từ chối version xung đột. `GradleLockError` ánh xạ về `MgError::Other` hiện có và giữ nguyên thông báo tiếng Anh.

`gradle_lock.rs` owns the public DTO and stable entry points. `parse/` and `collapse/` own their separate algorithms; tests cover both child owners plus the public compatibility path and error mapping.

`gradle_lock.rs` sở hữu DTO công khai và entry point ổn định. `parse/` và `collapse/` lần lượt sở hữu thuật toán; test bao phủ hai owner con, đường tương thích công khai và ánh xạ lỗi.
