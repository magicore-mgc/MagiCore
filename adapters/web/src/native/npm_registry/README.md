# npm_registry

Stable native npm registry API facade. `mod.rs` declares owner modules and exposes the original names; `npm_registry.rs` re-exports the public API.

Facade API npm native ổn định. `mod.rs` khai báo các module owner, `npm_registry.rs` tái xuất các tên public cũ.

`mod.rs` declares the same-name implementation module. The existing cross-owner test contract remains in `native/test/npm_registry_tests.rs` during the source-only phase; test splitting/relocation is deferred until source decomposition across the repository is complete.

`mod.rs` khai báo module triển khai cùng tên. Test contract hiện có vẫn ở `native/test/npm_registry_tests.rs` trong giai đoạn chỉ bóc tách source; việc tách/chuyển test để sau khi hoàn tất toàn bộ source repository.
