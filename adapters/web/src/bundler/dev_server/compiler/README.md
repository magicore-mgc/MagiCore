# esbuild compiler owner / Owner compiler esbuild

Owns the native esbuild invocation, output selection and the browser-safe JavaScript error payload. Compiler diagnostics are typed locally before the route renders them.

Sở hữu lời gọi esbuild native, chọn output và payload lỗi JavaScript an toàn cho browser. Chẩn đoán compiler có kiểu cục bộ trước khi route render.

The compiler remains transpile-only (bundle = false) and keeps the locked compiler identity in the compilation key.

Compiler tiếp tục chỉ transpile (bundle = false) và giữ danh tính compiler đã khóa trong compilation key.
