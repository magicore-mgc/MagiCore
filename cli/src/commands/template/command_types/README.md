# Command Types

## Trách nhiệm tiếng Việt

Khai báo các biến thể lệnh Clap và kiểu tham số; giữ nguyên tên lệnh, option và mặc định.

## Responsibility / Trách nhiệm

Clap command variants and arguments only; command spelling and defaults remain unchanged.

## Boundaries / Ranh giới

**English:** Errors reuse the existing shared constructors, types, messages and propagation paths.

**Tiếng Việt:** Lỗi dùng lại constructor, kiểu, thông điệp và cách truyền lỗi hiện có; không tạo error placeholder.

## Ownership / Sở hữu

- English: mod.rs exports this owner surface to its parent.
- Tiếng Việt: mod.rs export API của owner lên module cha.
- English: the same-name Rust file contains implementation.
- Tiếng Việt: file Rust cùng tên chứa phần triển khai.
- English: this folder adds no network, cache, fallback or CLI behavior.
- Tiếng Việt: thư mục không thêm hành vi mạng, cache, fallback hay CLI mới.
## Implementation / Cài đặt

- [command_types.rs](command_types.rs)
