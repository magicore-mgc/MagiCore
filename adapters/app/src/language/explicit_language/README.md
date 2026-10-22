# Explicit App language

EN: Reads only a string at `[app].language` from a regular, parseable `mgc.toml`. `None` means no usable explicit string; a returned unknown string is deliberately handled by the coordinator as `None` without marker fallback.

VI: Chỉ đọc chuỗi `[app].language` từ `mgc.toml` là file thường và parse được. `None` nghĩa là không có chuỗi tường minh dùng được; chuỗi lạ được trả về để coordinator kết thúc nhận diện bằng `None`, không fallback marker.
