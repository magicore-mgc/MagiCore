# Shared content store / Content store dùng chung

Constructs the shared CAS used by native Flutter and Swift install. Keep store-root policy centralized here; lane-specific archive or checkout decisions remain in their lane owners. Errors use the shared `MgError::Store` adapter contract.

Khởi tạo CAS dùng chung cho install Flutter và Swift native. Quy tắc chọn root store tập trung tại đây; quyết định archive/checkout riêng của lane vẫn ở owner lane. Lỗi dùng hợp đồng adapter `MgError::Store` chung.
