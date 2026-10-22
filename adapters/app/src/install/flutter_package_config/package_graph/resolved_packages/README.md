# Resolved Flutter packages

Projects the MGC `ResolvedGraph` into Dart package-graph entries. It owns root-name collision checks, duplicate package-name rejection and sorted/deduplicated dependency-name projection; package resolution remains outside this owner.

## Trách nhiệm

Chuyển `ResolvedGraph` của MGC thành entry Dart package graph. Owner kiểm tra trùng tên root, từ chối package name trùng và sắp xếp/loại dependency name trùng; không sở hữu bước resolve package.
