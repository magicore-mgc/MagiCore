# Shared install profiling owner

This owner reads the existing `MAGICORE_WEB_PROFILE_INSTALL` setting and renders the established install timing line to stderr. It is shared because the measured add/remove/install stages are coordinated in the shared core command service. The environment name, truthy-value set, output prefix and duration source remain unchanged.

The operation is infallible: invalid or absent environment values disable output, and writing to stderr does not return a recoverable error. There is deliberately no `error.rs`; inventing a failure type would not add a real failure boundary. Local tests cover the pure parser without changing process-global environment state.

# Owner profiling install dùng chung

Owner này đọc cấu hình `MAGICORE_WEB_PROFILE_INSTALL` hiện có và in dòng thời gian install ra stderr theo format cũ. Đây là owner dùng chung vì các bước add/remove/install được đo do shared core command service điều phối. Tên biến môi trường, nhóm giá trị bật, prefix output và nguồn duration được giữ nguyên.

Operation này không có lỗi có thể khôi phục: env sai hoặc vắng mặt sẽ tắt output; ghi stderr không trả về lỗi. Chủ động không tạo `error.rs` vì type lỗi giả không tạo thêm boundary thật. Test cục bộ kiểm tra parser thuần mà không sửa environment toàn process.
