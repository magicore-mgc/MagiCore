# Go version materialization

Turns one selected Go version into a `ResolvedEntry`: validates `.info` identity, parses the required `.mod`, retains dependency and provenance markers, and selects the proxy ziphash or sumdb h1 marker. Keep HTTP status handling, metadata checks, marker order and error text stable. Shared `MgError` is the error contract; this owner does not create a duplicate error type.

## Tiếng Việt

Chuyển một version Go đã chọn thành `ResolvedEntry`: xác minh identity `.info`, parse `.mod` bắt buộc, giữ dependency/provenance marker và chọn ziphash của proxy hoặc marker h1 sumdb. Phải giữ nguyên xử lý HTTP status, kiểm tra metadata, thứ tự marker và thông báo lỗi. Owner dùng `MgError` chung, không tạo error type trùng. Test identity ở `test/`; contract HTTP đầy đủ ở integration target của crate.
