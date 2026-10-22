# parse

Owns parsing of `<name>[@<ref>]` and construction of the shared `ScaffoldSpec`. It preserves the original split-at-first-`@`, reference precedence, validation-before-normalization order, and public function signature.

Quản lý parse `<name>[@<ref>]` và tạo `ScaffoldSpec` dùng chung. Giữ nguyên việc tách tại `@` đầu tiên, thứ tự phân loại reference, validation trước normalize và chữ ký hàm public.

The accepted reference forms are exactly those documented by `ScaffoldRef`; this owner does not add a new package-manager or framework capability.
