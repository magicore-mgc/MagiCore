# NuGet resolution owner

This folder is the private implementation boundary for `RegistryProtocol for NuGetProtocol`. `resolve.rs` coordinates version choice, authoritative package integrity, nuspec dependency selection, artifact download and digest verification while preserving the existing resolver facade.

```text
resolve/
├── mod.rs
├── resolve.rs                     # Stable protocol coordinator + download/verify
├── version_selection/             # Flat-container listed-version choice
├── registration_integrity/         # Registration/listed gate + catalog SHA-512
└── dependency_graph/               # Nuspec groups + consumer TFM selection
```

Each behavior owner has a same-name implementation, thin `mod.rs`, nearest `test/`, and bilingual README. Shared outward failures remain `MgError`; these owners do not introduce a second error contract. The existing integration target remains `core/crates/mgc-resolver/tests/protocols_nuget.rs`. The added unit tests are source-only evidence until Cargo executes them.

## Tiếng Việt

Folder này là ranh giới implementation private cho `RegistryProtocol for NuGetProtocol`. `resolve.rs` điều phối chọn version, integrity package chính thống, chọn dependency nuspec, tải artifact và kiểm tra digest nhưng giữ facade resolver hiện tại.

Mỗi owner hành vi có implementation cùng tên, `mod.rs` wiring mỏng, `test/` gần owner và README song ngữ. Lỗi outward tiếp tục dùng `MgError` chung; không tạo contract lỗi thứ hai. Integration target giữ tại `core/crates/mgc-resolver/tests/protocols_nuget.rs`. Test unit vừa thêm mới là source evidence, chưa có kết quả chạy Cargo.
