# Gradle DSL policy owner

This owner selects the existing build file and constructs fail-closed parse/write errors. It never evaluates or rewrites Gradle DSL.
Owner này chọn build file hiện hữu và tạo lỗi từ chối parse/ghi theo nguyên tắc fail-closed. Nó không thực thi hoặc ghi lại DSL Gradle.

`.kts` priority, missing-file text, Unsupported capability names, and user guidance are compatibility contracts.
Ưu tiên `.kts`, nội dung lỗi khi thiếu file, tên capability Unsupported và hướng dẫn người dùng là hợp đồng tương thích.
