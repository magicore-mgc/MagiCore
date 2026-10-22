# App project identity

Owns the App project-root validation and manifest-based language selection shared by App dependency commands. It does not choose package lifecycle support; that remains with the dependency ownership gate.

`project_identity.rs` holds the behavior, `mod.rs` exports the two existing helpers, and `test/` contains the nearest language-detection test.

# Nhận diện project App

Sở hữu việc xác thực root project App và chọn ngôn ngữ từ manifest dùng chung cho các lệnh dependency App. Owner này không quyết định lane nào có lifecycle package; quyết định đó thuộc dependency ownership gate.

`project_identity.rs` chứa behavior, `mod.rs` export hai helper hiện có, `test/` chứa test nhận diện ngôn ngữ gần owner.
