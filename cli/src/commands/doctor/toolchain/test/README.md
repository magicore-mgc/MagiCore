# Toolchain tests

These nearest-owner tests pin the six inventory rows/argv, timeout/scope/clean-environment options and the environment allowlist. They cover missing and directory candidates, empty PATH, regular files, Unix non-Unicode PATH components, Windows `.exe` lookup, absolute-parent pinning, terminal-safe paths and bounded version parsing. They never launch a tool or depend on installed versions. Unix/Windows cases are cfg-gated. Tests are authored but NOT_RUN.

# Test toolchain

Các test gần owner khóa sáu hàng inventory/argv, timeout/scope/environment sạch và allowlist biến môi trường. Chúng bao phủ candidate thiếu hoặc là thư mục, PATH rỗng, file thường, PATH component không Unicode trên Unix, tìm `.exe` trên Windows, ghim parent tuyệt đối, path an toàn cho terminal và parse version có giới hạn. Test không chạy tool hay phụ thuộc version đã cài. Case Unix/Windows được cfg-gate. Test đã viết nhưng NOT_RUN.
