# Version-reference usage counter

This owner recursively counts TOML `version.ref` references so a single-package bump cannot silently alter a shared version pin.
Owner này đếm đệ quy `version.ref` trong TOML để thao tác bump một package không âm thầm đổi pin đang dùng chung.

The recursive table/array traversal and conservative count are unchanged from the prior helper.
Cách duyệt đệ quy table/array và phép đếm thận trọng được giữ nguyên từ helper cũ.
