# Import file mode tests / Test luồng import file

This folder verifies streamed-import ownership and cleanup behavior at the nearest source owner. Tests inject a reader failure after a partial write and assert no staging entry is left behind. Tests are authored here and are not considered passing until the crate test runs.

Thư mục này kiểm tra quyền sở hữu staging và cleanup của import dạng stream ngay cạnh source owner. Test tiêm lỗi reader sau khi đã ghi một phần và xác nhận không còn entry staging. Test đã viết nhưng chưa được tính là pass cho tới khi chạy test crate.
