# Shared cache lookup tests

Nearest tests for refusing cache graphs that do not identify one explicit direct root per manifest dependency, and for bypassing shared cache on fresh resolves or active age gates. This prevents a stale/malformed cache entry from relying on a name/range guess or short-circuiting the requested fresh/security path.

Test gần nhất cho việc từ chối graph cache không chỉ rõ một root trực tiếp cho mỗi dependency manifest và bỏ qua shared cache trong fresh resolve/age gate. Việc này ngăn cache cũ/sai dựa vào suy đoán tên/range hoặc short-circuit đường fresh/security được yêu cầu.
