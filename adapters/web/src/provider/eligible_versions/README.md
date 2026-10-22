# Builds the resolver candidate list and applies minimum-release-age. Missing timestamps are rejected unless the explicit policy allows them; if a non-empty registry has no candidate left, it returns the existing diagnostic. / Tạo danh sách candidate cho resolver và áp dụng minimum-release-age. Timestamp thiếu bị loại trừ trừ khi policy cho phép; nếu registry có version nhưng lọc hết, trả diagnostic hiện có.

## Responsibility

Builds the resolver candidate list and applies minimum-release-age. Missing timestamps are rejected unless the explicit policy allows them; if a non-empty registry has no candidate left, it returns the existing diagnostic. Public function remains `provider::eligible_versions`; it is the shared list-filtering path.

## Scope and boundary

This owner contains only this behavior. `mod.rs` wires the same-name implementation and its nearest tests. It uses shared resolver/package types and keeps the established provider API and errors.

## Phạm vi và ranh giới

Tạo danh sách candidate cho resolver và áp dụng minimum-release-age. Timestamp thiếu bị loại trừ trừ khi policy cho phép; nếu registry có version nhưng lọc hết, trả diagnostic hiện có. Giữ hàm public `provider::eligible_versions`; đây là đường lọc danh sách dùng chung.

Owner này chỉ chứa hành vi trên. `mod.rs` nối implementation cùng tên với test gần nhất. Owner dùng type resolver/package chung và giữ nguyên API provider cùng error contract.
