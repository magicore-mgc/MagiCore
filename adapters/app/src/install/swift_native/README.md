# App Swift native installer

This module coordinates registry archive installation and the current fail-closed handling for non-archive Git/local inputs. It does not execute SwiftPM or Git.

```text
swift_native/
├── mod.rs
├── swift_native.rs                 # Stable install coordinator
├── package_source/                  # Existing case-sensitive .zip classification
├── registry_archive/                # Download → verify → CAS → materialize
├── git_provenance/                  # First exact-name git-commit lock marker
├── git_checkout/                    # Defensive Unsupported protocol boundary
├── package_resolved_pins/           # Graph-order pin projection
│   ├── registry_pin/                # Registry identity/version fields
│   └── git_pin/                     # Git identity/revision/location fields
└── store_root/                      # Override/default path and directory creation
```

The existing App lifecycle integration test remains in `adapters/app/tests/native_swift_tests.rs`; it is NOT_RUN during this source-first refactor. Git/local Swift dependencies remain unsupported by the resolver and protocol. Nearest-owner source tests are authored but not yet executed.

## Tiếng Việt

Module này điều phối cài archive registry và xử lý fail-closed hiện có cho input Git/local không phải archive. Nó không chạy SwiftPM hoặc Git.

Cây folder bên trên chỉ ra owner: coordinator ổn định, phân loại source, chuỗi registry archive, provenance lock, guard Git, chiếu pin theo thứ tự graph và chọn gốc store. Pin registry và pin Git được tách riêng.

Integration test lifecycle App hiện giữ tại `adapters/app/tests/native_swift_tests.rs`; test NOT_RUN trong đợt refactor source-first này. Git/local Swift dependency hiện vẫn Unsupported từ resolver/protocol. Test source gần owner đã viết nhưng chưa chạy.
