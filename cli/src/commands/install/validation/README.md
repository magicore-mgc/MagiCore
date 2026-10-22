# Install request and ownership validation / Kiểm tra request và ownership

`validation.rs` enforces the 50-package limit before the adapter/core/ecosystem/framework/manifest-format ownership firewall. Toolchain-owned manifest writes are rejected. Cloud routing remains fail-closed when its feature is absent or detection is ambiguous. Existing gate errors are propagated from shared constructors. Tests live under [`test`](test/README.md).

`validation.rs` áp giới hạn 50 package trước cổng ownership adapter/core/ecosystem/framework/manifest. Ghi manifest do toolchain sở hữu bị chặn. Nhánh cloud fail-closed khi thiếu feature hoặc không nhận diện được. Lỗi dùng constructor chung; test nằm cạnh owner.
