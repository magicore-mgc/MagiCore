# Recursive workspace order

This owner builds the shared workspace graph and applies `mgc_workspace::dependency_first_levels`. It returns manifest-backed paths in dependency-first order and appends paths without a graph node in their original order. Graph and cycle errors propagate to the coordinator.

## Tiếng Việt

Owner này dựng graph workspace dùng chung và áp dụng `mgc_workspace::dependency_first_levels`. Nó trả path có manifest theo dependency-first rồi nối path không có graph node theo input order. Lỗi dựng graph và cycle được truyền lên coordinator.
