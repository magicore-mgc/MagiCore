# Implements the resolver version-prefetch path: use cached version lists first, then cached metadata, then batched metadata fetch; every produced list passes through the age gate. / Triển khai đường resolver prefetch version: dùng cache version, rồi metadata cache, cuối cùng fetch metadata theo batch; mọi danh sách đều qua age gate.

## Responsibility

Implements the resolver version-prefetch path: use cached version lists first, then cached metadata, then batched metadata fetch; every produced list passes through the age gate. A missing alias-keyed metadata result returns the existing `DependencyError`.

## Scope and boundary

This owner contains only this behavior. `mod.rs` wires the same-name implementation and its nearest tests. It uses shared resolver/package types and keeps the established provider API and errors.

## Phạm vi và ranh giới

Triển khai đường resolver prefetch version: dùng cache version, rồi metadata cache, cuối cùng fetch metadata theo batch; mọi danh sách đều qua age gate. Thiếu metadata theo alias trả `DependencyError` hiện có.

Owner này chỉ chứa hành vi trên. `mod.rs` nối implementation cùng tên với test gần nhất. Owner dùng type resolver/package chung và giữ nguyên API provider cùng error contract.
