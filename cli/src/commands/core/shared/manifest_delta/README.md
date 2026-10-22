# Manifest delta owner

This owner turns already-resolved package identities into an exact-pin manifest and merges the resulting graph into a prior lock-derived graph. It preserves existing package metadata when present, combines direct/dev flags, deduplicates dependency edges, and appends new packages in resolver order.

The owner does not contact registries, choose versions, enforce security policy, write manifests or lockfiles, or invoke an installer. The parent shared service controls when delta installation is allowed.

Local tests are registered from `mod.rs` and cover pin/group construction, graph merge precedence, edge deduplication, and typed error source preservation. Tests are authored but have not been run in this structural phase.

# Owner manifest delta

Owner này chuyển package identity đã resolve thành manifest pin chính xác và gộp graph tạo ra vào graph trước đó đọc từ lock. Owner giữ metadata package đã có, gộp cờ direct/dev, khử cạnh dependency trùng và nối package mới theo thứ tự resolver.

Owner không gọi registry, chọn version, thi hành security policy, ghi manifest/lockfile hoặc gọi installer. Shared service cha quyết định khi nào được dùng delta install.

Test cục bộ được đăng ký từ `mod.rs`, bao phủ tạo pin/nhóm dependency, thứ tự ưu tiên khi merge graph, khử cạnh trùng và giữ source của lỗi có kiểu. Test đã được viết nhưng chưa chạy trong phase refactor cấu trúc này.
