# Platformio Manifest IoT adapter owner

Parses only modeled PlatformIO env lib_deps. URL, git, file, symlink or otherwise unmodelable entries continue to return an error rather than a fabricated empty graph.

Chỉ parse lib_deps của env PlatformIO mà MGC mô hình được. URL, git, file, symlink hoặc entry không mô hình được tiếp tục trả lỗi thay vì tạo graph rỗng giả.

The owner preserves the current IoT capability matrix and reuses shared types/errors.

Owner giữ nguyên capability matrix IoT và dùng lại type/error chung.
