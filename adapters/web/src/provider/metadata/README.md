# Loads package metadata with a per-package async lock, maps aliases to source package names, and mirrors version-cache entries under both requested and source names. / Tải metadata với async lock theo package, ánh xạ alias sang source package và ghi version-cache dưới cả tên yêu cầu lẫn tên source.

## Responsibility

Loads package metadata with a per-package async lock, maps aliases to source package names, and mirrors version-cache entries under both requested and source names. Metadata transport/fallback policy remains in shared cache/native registry owners.

## Scope and boundary

This owner contains only this behavior. `mod.rs` wires the same-name implementation and its nearest tests. It uses shared resolver/package types and keeps the established provider API and errors.

## Phạm vi và ranh giới

Tải metadata với async lock theo package, ánh xạ alias sang source package và ghi version-cache dưới cả tên yêu cầu lẫn tên source. Transport/fallback metadata tiếp tục thuộc owner cache/native registry chung.

Owner này chỉ chứa hành vi trên. `mod.rs` nối implementation cùng tên với test gần nhất. Owner dùng type resolver/package chung và giữ nguyên API provider cùng error contract.
