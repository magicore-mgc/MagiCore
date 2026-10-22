# Storage tests

The isolated temporary-directory tests cover missing-path non-mutation, actual write-probe cleanup, non-directory paths and nearest-existing-ancestor selection. They avoid changing global user permissions; a reliable read-only case is deferred to platform-specific CI.

# Test storage

Các test thư mục tạm cô lập kiểm tra path thiếu không bị tạo, write probe được dọn, path không phải directory và chọn ancestor gần nhất đang tồn tại. Test không đổi permission toàn cục của user; ca read-only ổn định được để cho CI riêng theo OS.
