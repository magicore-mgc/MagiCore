# Store and disk snapshot

This owner resolves the configured global store root, checks actual write access with a private temporary file that must be removed successfully, and reads free space through the nearest existing directory ancestor. Inspection never creates the store directory. A cleanup failure is returned as a typed storage error and makes the store probe fail.

# Snapshot store và disk

Owner này lấy global store root đã cấu hình, kiểm tra quyền ghi thật bằng file tạm riêng tư phải xóa thành công, và đọc dung lượng trống qua thư mục tổ tiên gần nhất đang tồn tại. Inspection phân biệt store thiếu với symlink bị đứt ở mọi thành phần của path, không tạo thư mục store. `--fix` chuẩn bị parent chain rồi tạo leaf nguyên tử; nếu path leaf đã xuất hiện, nó kiểm tra lại thay vì theo symlink hoặc ghi đè. Lỗi dọn file được trả thành typed storage error và khiến phép thử store thất bại.
