# Rust audit orchestration

EN: Reads Cargo.lock through the shared bounded-input owner and sends only supported crates.io pins to MGC's shared OSV audit function. Shared I/O and OSV errors propagate unchanged.

VI: Đọc Cargo.lock qua owner input có giới hạn và chỉ gửi pin crates.io được hỗ trợ sang hàm audit OSV dùng chung của MGC. Lỗi I/O và OSV dùng chung được propagate nguyên trạng.
