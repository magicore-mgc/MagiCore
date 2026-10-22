# Cache Contract tests

## Trách nhiệm tiếng Việt

Test version marker và manifest không đạt schema processor.

## Responsibility / Trách nhiệm

Existing version-marker and processor-manifest validation tests.

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

- [cache_contract.rs](cache_contract.rs)
