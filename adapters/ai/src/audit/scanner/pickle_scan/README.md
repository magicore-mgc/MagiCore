# Pickle scanner

This owner scans pickle-like model files in 64 KiB chunks, retaining enough trailing bytes to find signatures split across reads. It emits Critical findings for configured byte signatures and a High finding for the pre-existing large-file threshold. I/O errors propagate through MgResult.

Owner này quét file model dạng pickle theo chunk 64 KiB, giữ phần byte cuối để phát hiện signature bị chia giữa các lần đọc. Nó tạo finding Critical cho byte signature đã cấu hình và finding High khi vượt ngưỡng file lớn hiện có. Lỗi I/O được truyền qua MgResult.

This is heuristic detection. A clean result means only that these byte signatures and the existing size rule did not match; it is not proof that deserializing the file is safe. Tests live beside this owner in test/.

Đây là phát hiện heuristic. Kết quả sạch chỉ có nghĩa là các byte signature và quy tắc kích thước hiện có không khớp; nó không chứng minh deserialize file là an toàn. Test đặt gần owner trong test/.
