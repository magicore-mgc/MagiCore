# Publish summary owner

Owns the serializable result DTO consumed by the CLI summary renderer. It formats no console output and does not upload packages.

Sở hữu DTO kết quả có thể serialize mà CLI summary renderer sử dụng. Owner không in console và không upload package.

The original module note describes this DTO as a migration from the CLI publish command for later pipeline reuse (Phase 3+).

Ghi chú module cũ mô tả DTO này được chuyển từ CLI publish command để pipeline tái sử dụng về sau (Phase 3+).

The existing `Serialize`-only contract and public fields remain unchanged; the nearest test checks the emitted field names and values.

Contract hiện tại chỉ có `Serialize` và các field public được giữ nguyên; test gần nhất kiểm tra tên field và giá trị được xuất ra.
