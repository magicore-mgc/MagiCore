# Parent directory sync / Đồng bộ thư mục cha

Owns the existing Unix directory fsync and the documented no-op on other platforms. Rename publication and call-site ordering remain with their callers.

Owner này giữ fsync thư mục trên Unix và no-op đã ghi rõ trên platform khác. Việc publish rename và thứ tự tại call site vẫn thuộc caller.
