# Swift install owner / Owner install Swift

EN: Owns the native Swift install call, forwards the resolver's retained `SwiftRegistryProtocol`, and preserves the existing replay-versus-lock-write condition. Protocol resolution and archive verification remain in their current owners. `test/` freezes lock-write eligibility.

VI: Sở hữu lời gọi install Swift native, chuyển tiếp `SwiftRegistryProtocol` được giữ từ resolver và bảo toàn điều kiện replay/ghi lock hiện có. Resolve protocol và xác minh archive vẫn ở owner hiện tại. `test/` khóa điều kiện ghi lock.
