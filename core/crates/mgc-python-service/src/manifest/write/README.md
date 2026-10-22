# Python project writing

`write.rs` updates only the base dependency list and the selected `dependency-groups.dev` field. It preserves unrelated project metadata, refuses dependency-group layouts outside the native lane, and delegates regular-file/symlink safety plus atomic replacement to `mgc-adapter-base`. `error.rs` classifies local rejection/serialization cases; `MgError::Anyhow` preserves the local type and TOML source chain while keeping the public `MgResult` boundary.

The writer does not generate a lockfile or run an external Python package manager.

## Trách nhiệm

`write.rs` chỉ cập nhật dependency nền và `dependency-groups.dev`. Module giữ metadata project không thuộc MGC, từ chối group ngoài lane native, và giao bảo vệ regular file/symlink cùng atomic replace cho `mgc-adapter-base`. `error.rs` phân loại lỗi từ chối/serialize cục bộ; `MgError::Anyhow` giữ local type/source chain TOML và vẫn giữ ranh giới `MgResult`.

Writer không sinh lockfile và không chạy package manager Python bên ngoài.
