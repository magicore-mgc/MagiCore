# types

Index of npm metadata, registry state, tarball result and scoped-client state models. `types.rs` re-exports the model owners.

Mục lục metadata npm, trạng thái registry, kết quả tarball và trạng thái scoped client. `types.rs` tái xuất các owner kiểu dữ liệu.

`mod.rs` declares the same-name implementation module. The existing cross-owner test contract remains in `native/test/npm_registry_tests.rs` during the source-only phase; test splitting/relocation is deferred until source decomposition across the repository is complete.

`mod.rs` khai báo module triển khai cùng tên. Test contract hiện có vẫn ở `native/test/npm_registry_tests.rs` trong giai đoạn chỉ bóc tách source; việc tách/chuyển test để sau khi hoàn tất toàn bộ source repository.
