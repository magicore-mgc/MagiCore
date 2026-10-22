# Dependency ownership projection / Projection dependency ownership

This owner converts the existing `dep_gate::owner_for` decisions into the JSON contract consumed by CLI users, MCP and lifecycle-matrix checks. It never decides whether an operation is native itself. The operation projection calls the existing gate for every `DepOp`; language and manifest projections only expose differing, manifest-qualified results.

Owner này chuyển quyết định `dep_gate::owner_for` hiện có thành JSON contract mà CLI, MCP và lifecycle matrix dùng. Nó không tự quyết định operation nào là native. Projection operation gọi gate hiện có cho mọi `DepOp`; projection language/manifest chỉ xuất khác biệt đã gắn điều kiện manifest.

`mgc-native`, `scaffold-only`, and `unsupported` are the closed output vocabulary. No third-party tool is allowed to appear as the owner of a native cell. Existing assertions live in this folder's `test/` and remain source-only until the storage gate permits execution.

`mgc-native`, `scaffold-only` và `unsupported` là vocabulary output đóng. Tool bên thứ ba không được xuất hiện như owner của cell native. Assertion hiện có nằm tại `test/` và mới chỉ là source cho tới khi điều kiện dung lượng cho phép chạy.
