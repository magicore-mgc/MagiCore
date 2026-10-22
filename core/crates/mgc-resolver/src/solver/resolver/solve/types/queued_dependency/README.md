# QueuedDependency

## Responsibility / Trách nhiệm
Carries package name and spec through the resolver queue while retaining whether the request originated in the project's root manifest and its root index.

Mang tên package và spec xuyên queue resolver, đồng thời giữ nguồn request là manifest root cùng index tương ứng.

## Tests / Kiểm thử
Nearest tests distinguish indexed root requests from unindexed transitive requests. NOT_RUN.
