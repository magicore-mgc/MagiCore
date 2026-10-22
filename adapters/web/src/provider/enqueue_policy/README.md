# Implements optional dependency admission: cached decisions win, known native packages are filtered to the host, required edges stay enabled, and optional edges are checked against selected version metadata. / Quyết định enqueue dependency optional: dùng quyết định cache trước, lọc package native theo host, giữ required edge, và kiểm tra optional edge bằng metadata version đã chọn.

## Responsibility

Implements optional dependency admission: cached decisions win, known native packages are filtered to the host, required edges stay enabled, and optional edges are checked against selected version metadata. Age-gate failures continue to propagate; missing optional candidates stay excluded.

## Scope and boundary

This owner contains only this behavior. `mod.rs` wires the same-name implementation and its nearest tests. It uses shared resolver/package types and keeps the established provider API and errors.

## Phạm vi và ranh giới

Quyết định enqueue dependency optional: dùng quyết định cache trước, lọc package native theo host, giữ required edge, và kiểm tra optional edge bằng metadata version đã chọn. Lỗi age gate tiếp tục được truyền lên; thiếu candidate optional vẫn bị loại.

Owner này chỉ chứa hành vi trên. `mod.rs` nối implementation cùng tên với test gần nhất. Owner dùng type resolver/package chung và giữ nguyên API provider cùng error contract.
