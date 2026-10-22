# body_stream

Consumes a registry response incrementally. Bodies up to 2 MiB remain in memory; when the observed body crosses that threshold, the buffered prefix and remaining chunks are written to a create-new staging file. The decision uses the observed stream, so a missing or understated `Content-Length` cannot route a large body through the full-body `Response::bytes()` allocation. A single transport chunk is still owned by the HTTP stream while it is processed.

Đọc response theo từng chunk. Body tối đa 2 MiB được giữ trong RAM; khi dữ liệu thực tế vượt ngưỡng, phần đã đệm và các chunk sau được ghi xuống file staging tạo mới. Quyết định dựa trên stream thực tế nên `Content-Length` thiếu hoặc khai thấp không còn khiến toàn body đi qua `Response::bytes()`. Một chunk đơn lẻ vẫn do HTTP stream cấp phát trong lúc xử lý.

`cleanup/` owns cancellation-safe deletion of partial files, `error/` owns typed failures, and `test/` owns focused stream boundary and cleanup tests. The public download API stays in the parent automatic owner.

`cleanup/` sở hữu việc xóa file dở dang an toàn khi hủy tác vụ, `error/` sở hữu lỗi có kiểu, `test/` chứa test ranh giới stream và dọn file. API tải công khai vẫn nằm ở owner automatic cha.
