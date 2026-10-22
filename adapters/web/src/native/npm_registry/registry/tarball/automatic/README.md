# automatic

Uses the existing 2 MiB threshold to return small tarballs in memory and stream large tarballs to disk.

Dùng ngưỡng 2 MiB hiện có: package nhỏ trả về RAM, package lớn ghi stream xuống đĩa.

`mod.rs` declares the same-name implementation module. The automatic body-stream owner has focused nearest-owner tests; registry authentication and HTTP contract tests remain in `native/test/npm_registry_tests.rs`.

`mod.rs` khai báo module triển khai cùng tên. Owner body-stream có test riêng gần source; test contract HTTP và xác thực registry vẫn ở `native/test/npm_registry_tests.rs`.
