# Project execution profile owner

This owner defines the serialized execution profile fields, their deserialization defaults, and the existing ecosystem-to-profile constructor policy. `project.rs` re-exports `ProjectExecutionConfig` at the existing public path and uses the private default resolver. No config schema or load/save behavior is owned here.

The profile mapping is intentionally kept as-is: Web uses the compatibility shell with JS/TS metadata and frontend/backend/WASM targets; Game/App/Lib use the native-ready binary lane; other ecosystems use the current empty-target fallback. This is configuration metadata, not evidence that each target is implemented.

This owner is pure and infallible, so there is no local `error.rs`. Nearest tests cover the defaults and remain separate from existing crate integration tests.

# Owner execution profile của project

Owner này khai báo các trường execution profile có serialize, giá trị mặc định khi deserialize và policy chọn profile theo ecosystem hiện tại. `project.rs` re-export `ProjectExecutionConfig` tại public path cũ và dùng resolver mặc định private. Schema config và hành vi load/save không thuộc owner này.

Mapping được giữ nguyên: Web dùng compatibility shell với metadata JS/TS và target frontend/backend/WASM; Game/App/Lib dùng lane native-ready với binary; ecosystem khác dùng fallback empty-target hiện tại. Đây là metadata cấu hình, không chứng minh target đã được triển khai.

Owner này thuần và không có lỗi có thể phát sinh nên không có `error.rs`. Test gần owner bao phủ default và vẫn tách khỏi test integration của crate.
