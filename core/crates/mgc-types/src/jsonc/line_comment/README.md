# JSONC line-comment scanner

Scans each logical line and removes `//` only outside a JSON string. Escape state is honored so an escaped quote cannot expose the remainder of a string as a comment.

Quét từng dòng logic và chỉ bỏ `//` bên ngoài JSON string. Theo dõi escape để dấu quote được escape không làm phần còn lại của string bị nhận nhầm là comment.
