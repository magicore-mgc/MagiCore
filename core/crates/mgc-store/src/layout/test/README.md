# Store layout tests

<!-- English test guide: scope and execution status. -->
<!-- Hướng dẫn test tiếng Việt: phạm vi và trạng thái chạy. -->

These tests verify the public path mapping contract without touching disk. They are registered through `layout/test/mod.rs` and are intentionally separate from production source.

Các test kiểm tra hợp đồng ánh xạ đường dẫn công khai mà không truy cập đĩa. Test được nối qua `layout/test/mod.rs` và tách khỏi source production.

Status: authored, NOT_RUN until the repository-wide source-refactor phase closes.
