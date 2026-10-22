# Tests for Windows executable identity

EN: These owner-local tests verify bounded streaming SHA-256 against the reference digest at empty, single-byte, buffer-boundary, and multi-buffer sizes, and verify that a missing executable returns an error. They exercise the existing helper without changing production identity policy. Windows-only behavior must be executed on Windows during the later verification phase.

VI: Test gần owner này đối chiếu SHA-256 streaming có giới hạn với digest chuẩn ở kích thước rỗng, một byte, đúng biên buffer và nhiều buffer; đồng thời xác nhận executable bị thiếu trả về lỗi. Test gọi helper hiện có và không đổi policy định danh production. Hành vi riêng Windows phải được chạy trên Windows ở giai đoạn verify sau.

`mod.rs` registers the same-name test module and exposes only the owner helper needed by the child test file.
`mod.rs` đăng ký module test cùng tên và chỉ nối helper owner mà file test con cần dùng.
