# Package domain / Miền kiểu package

This folder is the stable `mgc_types::package` namespace. Its root `mod.rs` is the namespace aggregator and intentionally has no `package.rs` implementation: it exports the four existing public types. Each behavior owner lives in its same-name folder with nearest-owner tests. The shared `MgError` remains the error contract. The root is the one structural exception to the same-name implementation rule because the public namespace has no single behavior to own.

Folder này là namespace ổn định `mgc_types::package`. `mod.rs` gốc chỉ gom module và export bốn type công khai hiện có, cố ý không có `package.rs` triển khai: namespace này không có một hành vi đơn lẻ để đặt vào file cùng tên. Mỗi owner hành vi nằm trong folder cùng tên và có test gần owner. `MgError` dùng chung vẫn là hợp đồng lỗi.

This extraction is source-only. The owner test files are present for later execution; no compiler, test runner, formatter, or CI result is implied here.

Đợt tách này chỉ xử lý source. Các file test owner được đặt sẵn để chạy ở pha sau; tài liệu này không hàm ý compiler, test runner, formatter hoặc CI đã chạy.
