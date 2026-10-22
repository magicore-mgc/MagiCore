# Dispatch types

This module preserves the stable `crate::dispatch::types` namespace and re-exports the three command contracts. Each type and detector has a same-name owner. The two large enums remain whole declarations because splitting Rust enum variants would change the public type API. Project-core detection is isolated and never mutates configuration.

## Tiếng Việt

Module này giữ namespace ổn định `crate::dispatch::types` và tái xuất ba hợp đồng lệnh. Mỗi kiểu và bộ dò có owner cùng tên. Hai enum lớn được giữ nguyên khai báo vì chia variant Rust thành enum khác sẽ đổi API kiểu public. Nhận diện core được tách riêng và không tự sửa cấu hình.
