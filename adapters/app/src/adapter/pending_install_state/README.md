# pending_install_state

EN: Holds the resolved lock entries and optional Swift registry protocol as one mutex-protected handoff. `resolve` clears old state, writes one complete snapshot after success, and `install` takes it once; lock replay discards any previous snapshot, while direct installs and lock replay use the existing environment fallback.

VI: Giữ lock entries đã resolve và protocol registry Swift tùy chọn trong một state có mutex. `resolve` xóa state cũ, chỉ ghi snapshot đầy đủ sau khi thành công; `install` lấy snapshot đúng một lần. Lock replay loại bỏ snapshot cũ; direct install và lock replay dùng fallback từ môi trường như trước.
