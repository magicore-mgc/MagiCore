# Shared capability command / Lệnh capability dùng chung

This core-neutral CLI owner publishes capability declarations for the adapters enabled in the current binary. The CLI command and MCP `mgc://capabilities` resource use the same structured payload builder. The lifecycle matrix reads the dependency ownership projection to fail closed when a framework/operation cell is not declared native.

Đây là owner CLI trung lập giữa các core, công bố capability của adapter được bật trong binary hiện tại. Lệnh CLI và resource MCP `mgc://capabilities` dùng chung bộ dựng payload. Lifecycle matrix đọc phần dependency ownership để từ chối khi framework/operation chưa được khai báo native.

`everything_else: unsupported` and omitted feature-disabled core rows are intentional. A declaration is a machine-readable boundary, not evidence by itself that a package lifecycle works on a real OS. `dependency_ownership` projects `dep_gate::owner_for`; it must not duplicate or override that policy. Unknown selected core names retain the stable `Unknown core '<name>'` error.

`everything_else: unsupported` và việc bỏ qua core không được compile là có chủ đích. Khai báo chỉ là ranh giới máy đọc, tự nó không chứng minh lifecycle package chạy trên OS thật. `dependency_ownership` chiếu từ `dep_gate::owner_for`, không sao chép hay ghi đè policy. Tên core được chọn nhưng không biết vẫn giữ lỗi `Unknown core '<name>'`.

See the child README files for core catalog, dependency projection and error ownership. Unit and binary E2E test sources are adjacent to their behavior owners; all newly authored/moved test sources remain unexecuted until the storage gate permits verification.
