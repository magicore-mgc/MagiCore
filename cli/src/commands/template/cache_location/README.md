# Cache Location

## Trách nhiệm tiếng Việt

Xác định thư mục cache template theo user và biến môi trường override.

## Responsibility / Trách nhiệm

User data/cache root resolution, including the current environment override.

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

- [cache_location.rs](cache_location.rs)
