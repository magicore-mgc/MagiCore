# Gradle manifest owner

This owner keeps the public `manifest::gradle` API stable while separating executable-DSL refusal, version-catalog parsing, safe pin updates, and local error conversion.
Owner này giữ ổn định API `manifest::gradle`, đồng thời tách việc từ chối DSL thực thi, đọc version catalog, cập nhật pin an toàn và chuyển đổi lỗi cục bộ.

Gradle build files remain executable programs and are never rewritten as generic dependency manifests. Only the explicitly implemented `gradle/libs.versions.toml` pin operation is available here. See each child README for its exact responsibility; current parser limitations are tracked in the S233 batch.
File build Gradle vẫn là chương trình có thể thực thi và không bị ghi lại như manifest phụ thuộc chung. Chỉ thao tác pin được triển khai tường minh trên `gradle/libs.versions.toml` được hỗ trợ. README từng owner con nêu trách nhiệm cụ thể; giới hạn parser hiện tại được ghi trong batch S233.
