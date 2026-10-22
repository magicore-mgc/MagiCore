# Archive Extract tests

## Trách nhiệm tiếng Việt

Test thay cache, archive traversal, manifest lỗi và archive có link.

## Responsibility / Trách nhiệm

Existing valid, traversal, malformed and link archive tests plus archive builders.

## Boundaries / Ranh giới

**English:** Cases and assertions are retained from the original module; shared fixtures are documented at [support/README.md](../support/README.md).

**Tiếng Việt:** Các case và assertion được giữ nguyên; fixture dùng chung được mô tả tại [support/README.md](../support/README.md).

## Ownership / Sở hữu

- English: mod.rs exports this owner surface to its parent.
- Tiếng Việt: mod.rs export API của owner lên module cha.
- English: the same-name Rust file contains implementation.
- Tiếng Việt: file Rust cùng tên chứa phần triển khai.
- English: this folder adds no network, cache, fallback or CLI behavior.
- Tiếng Việt: thư mục không thêm hành vi mạng, cache, fallback hay CLI mới.
## Implementation / Cài đặt

- [archive_extract.rs](archive_extract.rs)
