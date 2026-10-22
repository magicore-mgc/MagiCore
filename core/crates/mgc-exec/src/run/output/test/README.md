# Output conversion tests

`output.rs` verifies the diagnostic line cap and ordering, the byte cap for both output modes, and the distinction that full scanner capture is not line-capped; `error.rs` verifies that pipe-read failures retain their I/O source.

# Test chuyển đổi output

`output.rs` kiểm tra giới hạn số dòng và thứ tự của tail lỗi, giới hạn byte của cả hai chế độ output, đồng thời xác nhận capture đầy đủ cho scanner không bị giới hạn theo dòng; `error.rs` kiểm tra lỗi đọc pipe còn giữ nguồn I/O.

`mod.rs` only includes `output.rs` so the test module keeps its original parent scope.
`mod.rs` chỉ include `output.rs` để giữ nguyên scope cha của test module.
