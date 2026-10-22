# Gradle version-catalog reader

This owner reads `gradle/libs.versions.toml`, parses resolvable library pins, and skips unsupported or incomplete entries as before.
Owner này đọc `gradle/libs.versions.toml`, parse các pin library resolve được và tiếp tục bỏ qua entry thiếu hoặc chưa hỗ trợ như trước.

The parser is intentionally separate from executable Gradle DSL parsing. Its current requirement for a `[versions]` table and its `Option` error behavior are preserved in this structural slice.
Parser được tách khỏi việc xử lý DSL Gradle thực thi. Lát cấu trúc này giữ nguyên yêu cầu hiện tại về bảng `[versions]` và cách biểu diễn lỗi bằng `Option`.
