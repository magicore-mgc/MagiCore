# Python core ownership

`core_owner.rs` resolves a project's core from the existing marker/config policy, canonicalizes the legacy `cloud` spelling to `clo`, and filters Python lock entries by owner. Unowned legacy Python entries are accepted only for `lib`; a non-Lib owner receives the existing ambiguity error. This module does not rewrite markers or locks.

## Core owner Python

`core_owner.rs` xác định core của project theo policy marker/config hiện tại, chuẩn hóa alias `cloud` thành `clo`, và lọc entry Python theo owner. Entry Python legacy chưa gán owner chỉ được nhận ở `lib`; core khác nhận lỗi mơ hồ như trước. Module không ghi lại marker hay lock.
