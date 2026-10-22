# Kotlin OWASP fixture parser

## Responsibility

Holds the `cfg(test)` OWASP report schema/parser and eight nearest parser regression cases. Package URL parsing, severity mapping and UTF-8 truncation are separate owners. The former filesystem-only “stale report” test was removed because it called only `std::fs::remove_file` and never exercised application behavior; actual scanner freshness coverage remains absent and must be added in the later product-test phase. This is test support, not the production Kotlin audit implementation.

## Trách nhiệm

Chứa schema/parser report OWASP `cfg(test)` và tám case hồi quy parser gần owner. Parse package URL, ánh xạ severity và cắt UTF-8 là các owner riêng. Test “stale report” cũ đã bị xóa vì chỉ gọi `std::fs::remove_file`, không kiểm tra hành vi ứng dụng; hiện chưa có test freshness thực sự của scanner và cần bổ sung trong phase product-test sau. Đây là test support, không phải audit Kotlin production.
