# Registry protocol integration tests

The Cargo test target remains `tests/protocols.rs`. This folder separates public npm fail-closed behavior, shared trait defaults and graph-resolution behavior. The graph protocols are deterministic local mocks; they do not contact external registries.

Cargo test target vẫn là `tests/protocols.rs`. Thư mục tách kiểm tra npm fail-closed, mặc định trait dùng chung và graph. Graph dùng mock cục bộ xác định, không gọi registry bên ngoài.
