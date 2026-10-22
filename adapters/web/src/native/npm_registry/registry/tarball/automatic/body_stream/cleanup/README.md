# cleanup

Owns a cancellation-safe guard for a staging file created by the automatic tarball body reader. It is armed only after `create_new` succeeds, so an existing destination is never deleted by a failed attempt.

Sở hữu guard dọn file staging an toàn khi tác vụ tải bị hủy. Guard chỉ được bật sau khi `create_new` thành công nên lần tải lỗi không xóa file đích đã tồn tại từ trước.
