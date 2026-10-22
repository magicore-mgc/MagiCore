# HybridClient / HTTP client kết hợp

Owns online GET, explicit offline mode, and the current offline fallback path. It intentionally keeps the existing behavior that successful responses are not persisted to the offline cache.

Quản lý GET online, chế độ offline tường minh và đường fallback hiện tại. Owner giữ nguyên hành vi response thành công chưa được lưu vào cache offline.

The nearest-owner test seeds a cache and selects explicit offline mode, proving that branch can return cached bytes without network traffic. It does not claim the network fallback cache is populated.

Test gần owner tạo cache sẵn rồi chọn offline mode để xác nhận nhánh đó trả dữ liệu không cần mạng. Test không khẳng định cache fallback online được nạp.
