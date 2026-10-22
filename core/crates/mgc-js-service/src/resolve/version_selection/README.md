# Version selection owner / Owner chọn version

This owner holds the existing exact-range predicate and the rule that prefers a compatible project-root resolution before falling back to the newest compatible candidate.

Owner này chứa predicate exact range và rule ưu tiên resolution tương thích đã có ở project root trước khi chọn candidate tương thích mới nhất.

Nearest tests cover exact versus ranged constraints and root preference. The helper does not perform registry access or choose a package outside the supplied candidates.

Test gần nhất kiểm tra exact range so với range rộng và ưu tiên root. Helper không gọi registry và không chọn package ngoài candidates được truyền vào.
