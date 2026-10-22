# argv_text

Owns the shared strict UTF-8 view of one OS argument. Missing or non-UTF-8 values return None because callers cannot safely classify them as text; this helper performs no I/O and has no separate error domain.

Sở hữu cách đọc UTF-8 nghiêm ngặt cho một OS argument. Giá trị thiếu hoặc không phải UTF-8 trả về None vì caller không thể phân loại an toàn dưới dạng text; helper không I/O và không có miền lỗi riêng.

Callers are runtime parser owners and invocation-shape parsing. The nearest unit test is in test/argv_text.rs.
