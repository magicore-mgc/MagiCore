# Authorization header value

EN: Converts selected credentials to the existing Bearer or Basic header string.
VI: Chuyển credentials đã chọn thành chuỗi header Bearer hoặc Basic hiện tại.

Boundary: Bearer continues to take precedence; Basic uses standard Base64 of username:password.
Tests: test/header_value.rs covers Bearer, Basic, and absent credentials.
