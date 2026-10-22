# Atomic write if changed / Ghi atomic khi có thay đổi

Owns byte comparison, regular-file validation, and the no-op-on-identical contract before delegating changed content to `atomic_write`.

Sở hữu so sánh byte, kiểm tra file thường và contract không làm gì khi byte giống nhau trước khi giao nội dung thay đổi cho `atomic_write`.
