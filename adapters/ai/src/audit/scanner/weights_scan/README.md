# Model weights scanner

This owner reports the existing greater-than-100-GiB size warning and routes .pt, .pth, and .bin files to the pickle signature scanner. It preserves the original case-insensitive extension check and propagates scanner/I/O errors through MgResult.

Owner này báo ngưỡng kích thước lớn hơn 100 GiB hiện có và chuyển file .pt, .pth, .bin sang pickle signature scanner. Nó giữ kiểm tra đuôi không phân biệt hoa thường và truyền lỗi scanner/I/O qua MgResult.

The size finding is advisory, not a hard resource cap. Tests for dispatch and cross-chunk signatures live in test/.

Finding về kích thước chỉ mang tính cảnh báo, không phải giới hạn tài nguyên cứng. Test dispatch và signature qua ranh giới chunk nằm trong test/.
