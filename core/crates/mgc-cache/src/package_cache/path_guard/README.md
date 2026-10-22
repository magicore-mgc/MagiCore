# Cache path guard / Rào path cache

Creates/validates each cache directory below the trusted root without accepting child symlinks, rejects non-regular cached files and removes final symlinks as links instead of following them.
Tạo/kiểm tra từng thư mục cache dưới root tin cậy, không chấp nhận symlink con, từ chối file cache không thường và xóa symlink cuối như link thay vì đi theo.

This does not claim race-free protection against another process with write access to the cache root between validation and filesystem use.
Module này không tuyên bố chống race tuyệt đối nếu process khác có quyền ghi cache root giữa lúc validate và thao tác filesystem.
