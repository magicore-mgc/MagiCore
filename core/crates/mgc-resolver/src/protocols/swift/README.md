# Swift registry protocol

The stable public module facade for SwiftPM registry resolution, manifest parsing, pins, version requirements and dependency edits. Implementation responsibilities live in the named owner folders below. Existing callers keep the protocols::swift API.

## Tiếng Việt

Facade module công khai ổn định cho resolve registry SwiftPM, parse manifest, pin, requirement version và sửa dependency. Trách nhiệm triển khai nằm ở các owner có tên bên dưới. Caller hiện tại tiếp tục dùng API protocols::swift. Tám owner dùng MgError chung của mgc-types, giữ nguyên loại lỗi và thông điệp; không bọc lại lỗi thành một enum Swift trùng lặp.

The former root-only Swift unit tests are now registered beside their owners: Package.swift/dump-package parsing, Package.resolved pins, version requirements, Git source policy, registry URL construction and requirement edits. The existing registry integration suite remains in `registry/test/`; all 15 relocated unit tests are still NOT_RUN in the source-first phase.

## Tiếng Việt

Các unit test Swift trước đây nằm riêng ở root nay được đăng ký cạnh owner tương ứng: parse Package.swift/dump-package, Package.resolved pins, version requirement, Git source policy, tạo registry URL và sửa requirement. Integration suite registry vẫn ở `registry/test/`; toàn bộ 15 unit test được chuyển hiện vẫn NOT_RUN trong phase source-first.
