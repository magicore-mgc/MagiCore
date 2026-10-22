# Deno lockfile parser

English: Parses the npm and JSR maps in Deno lock JSON. npm pin keys use the shared final-`@` splitter; JSR keys are recorded as skipped. Malformed JSON and versionless npm keys preserve their current error messages.

Tiếng Việt: Phân tích map npm và JSR trong JSON lock Deno. Key npm dùng bộ tách tại dấu `@` cuối; key JSR được ghi nhận là skipped. JSON lỗi và key npm thiếu version giữ thông điệp lỗi hiện tại.
