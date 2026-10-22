# Python runtime compatibility

`runtime_compatibility.rs` delegates site-directory validation and wheel tag/runtime compatibility to `PypiProtocol`. Compiled wheels that do not match the selected host Python ABI and source distributions remain rejected with the existing guidance; no external package manager is invoked. Resolver errors pass through unchanged, while this owner's incompatibility error maps to the prior `MgError::Other` message.

## Tương thích runtime Python

`runtime_compatibility.rs` ủy quyền xác thực thư mục site và tag/runtime wheel cho `PypiProtocol`. Compiled wheel không khớp ABI Python host và source distribution vẫn bị từ chối với hướng dẫn cũ; không gọi package manager ngoài. Lỗi resolver giữ nguyên, lỗi không tương thích của owner này ánh xạ về `MgError::Other` như trước.
