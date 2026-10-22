# Verified Blob Tests

**Purpose:** Exercise the private crate-local verification route and memo contract.

**Mục đích:** Kiểm tra route xác minh nội bộ crate và hợp đồng memo.

The sources assert one miss followed by one identity-backed hit and ensure a corrupted stored blob returns `HashMismatch` while producing quarantine evidence. They remain NOT_RUN until the full source pass is complete.

Test source xác nhận một miss rồi một hit dựa trên identity, đồng thời bảo đảm blob bị sửa trả `HashMismatch` và tạo bằng chứng cách ly. Chưa chạy đến khi hoàn tất refactor source toàn repo.
