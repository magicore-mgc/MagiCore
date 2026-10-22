# OCI chunked upload / Upload OCI theo chunk

This owner provides the stable public upload module and re-exports UploadSession and ChunkedUploader. Its children own the serialized session DTO, request operations, file orchestration, local typed failures and nearest tests.

Owner này cung cấp module upload công khai ổn định và export UploadSession cùng ChunkedUploader. Các owner con quản lý DTO session, request, điều phối file, lỗi typed cục bộ và test gần source.

The current implementation does not persist or resume UploadSession state. Range validation, URL encoding, offset edge cases, chunk-size validation, repeated file reads and POST retry policy remain explicit behavior work.

Implementation hiện chưa lưu bền vững hoặc resume state UploadSession. Xác thực Range, encode URL, biên offset, kiểm tra chunk size, đọc file lặp và chính sách retry POST vẫn là đầu việc behavior riêng.
