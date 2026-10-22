# AI cache size

This owner reports logical bytes under a selected cache root by using the shared filesystem-size owner. A selected root symlink is canonicalized once; nested symlink entries are skipped. Missing roots report zero, while non-directory roots and filesystem failures return errors.

Owner này báo logical bytes dưới cache root bằng helper filesystem dùng chung. Root symlink được caller chọn sẽ canonicalize một lần; symlink bên trong bị bỏ qua. Root chưa tồn tại trả zero, root sai loại và lỗi filesystem trả error.

Reported bytes are a logical count, not guaranteed physical storage reclamation. Nearest source cases are in `test/cache_size.rs`.

Byte được báo là logical count, không đảm bảo bằng storage vật lý được thu hồi. Test source gần owner nằm tại `test/cache_size.rs`.
