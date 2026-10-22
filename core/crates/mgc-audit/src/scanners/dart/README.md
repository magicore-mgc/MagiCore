# Dart/Pub audit scanner

This owner adapts `pubspec.lock` to the shared OSV audit pipeline. The public functions remain available through `scanners::dart` and `scanners`.

Owner này chuyển `pubspec.lock` vào pipeline audit OSV dùng chung. Hai hàm public vẫn được giữ qua `scanners::dart` và `scanners`.

The parser intentionally preserves the current generated-lockfile subset and its tolerant behavior. It does not claim full YAML validation. The scanner does not own a separate error category; bounded-input and OSV failures keep their existing shared error paths.

Parser cố ý giữ tập con lockfile do pub sinh và cách xử lý dung sai hiện tại; không tuyên bố xác thực toàn bộ YAML. Scanner không có loại lỗi riêng; lỗi bounded-input và OSV tiếp tục theo luồng lỗi dùng chung.
