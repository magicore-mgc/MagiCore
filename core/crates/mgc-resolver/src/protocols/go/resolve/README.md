# Go resolution owner

The single `RegistryProtocol` implementation is the stable coordinator. It delegates exact-pin/list/@latest selection, one-version metadata materialization, multi-root MVS, artifact-byte download, and integrity verification to cohesive owners. Public `GoModProtocol` exports and the crate integration target stay at their existing paths.

Shared error and type contracts (`MgError`, `GoInfo`, `GoLatest`, `ResolvedEntry`) remain owned by their existing packages/modules; this folder adds no duplicate `error.rs` or types. Each behavior owner records invariants and nearest tests in its README.

## Tiếng Việt

`RegistryProtocol` chỉ được cài đặt một lần tại coordinator ổn định. Coordinator ủy quyền việc chọn exact-pin/list/@latest, dựng metadata một version, MVS nhiều root, tải bytes artifact và verify integrity cho các owner tương ứng. Export `GoModProtocol` và Cargo integration target giữ đường dẫn hiện tại.

Hợp đồng type/error dùng chung (`MgError`, `GoInfo`, `GoLatest`, `ResolvedEntry`) vẫn do package/module hiện hữu sở hữu; folder này không tạo `error.rs` hay type trùng. README từng owner ghi invariant và test gần nhất.
