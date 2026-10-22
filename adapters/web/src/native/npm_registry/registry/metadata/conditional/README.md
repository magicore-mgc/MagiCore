# conditional

Sends `If-None-Match`, handles `304 Not Modified`, and returns changed metadata with its new ETag.

Gửi `If-None-Match`, xử lý `304 Not Modified`, trả metadata mới cùng ETag.

`mod.rs` declares the same-name implementation module. The existing cross-owner test contract remains in `native/test/npm_registry_tests.rs` during the source-only phase; test splitting/relocation is deferred until source decomposition across the repository is complete.

`mod.rs` khai báo module triển khai cùng tên. Test contract hiện có vẫn ở `native/test/npm_registry_tests.rs` trong giai đoạn chỉ bóc tách source; việc tách/chuyển test để sau khi hoàn tất toàn bộ source repository.
