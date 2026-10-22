# registry_state

The per-instance `NpmRegistry` state: primary/fallback registries, tokens, scoped transports and age-gate mode.

Trạng thái `NpmRegistry` theo instance: registry chính/dự phòng, token, scoped transport và chế độ age-gate.

`mod.rs` declares the same-name implementation module. The existing cross-owner test contract remains in `native/test/npm_registry_tests.rs` during the source-only phase; test splitting/relocation is deferred until source decomposition across the repository is complete.

`mod.rs` khai báo module triển khai cùng tên. Test contract hiện có vẫn ở `native/test/npm_registry_tests.rs` trong giai đoạn chỉ bóc tách source; việc tách/chuyển test để sau khi hoàn tất toàn bộ source repository.
