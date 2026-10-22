# Shared support for this test target

`support.rs` owns the unique temporary-store path, fixed source digest, compiled-module builder, on-disk entry path, output-digest mirror and temp-file scanner. These helpers are shared only by this Cargo integration-test binary; they are deliberately separate from the same-named helper in `adversarial_integrity`.

`support.rs` sở hữu đường dẫn store tạm duy nhất, source digest cố định, builder module biên dịch, entry path trên đĩa, bản phản chiếu output digest và temp scanner. Helper chỉ dùng chung trong binary integration test Cargo này; giữ riêng với helper trùng tên ở `adversarial_integrity`.
