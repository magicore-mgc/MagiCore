# Dependency esbuild operation

This owner invokes the pinned esbuild implementation with the browser ESM options used by the dev server and selects its JavaScript/CSS outputs. Dependency builds enable a metafile and reject output if any reported input cannot be canonicalized inside the installed dependency roots. The check runs after esbuild reads inputs; it protects the returned/cache boundary and is not a filesystem sandbox.

Owner này gọi esbuild đã pin với option browser ESM dùng bởi dev server và chọn output JavaScript/CSS. Dependency build bật metafile và từ chối output nếu input được báo không canonicalize được trong root dependency đã cài. Check chạy sau khi esbuild đọc input; nó bảo vệ boundary trả/cache nhưng không phải filesystem sandbox.
