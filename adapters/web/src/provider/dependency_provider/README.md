# Implements the shared resolver `DependencyProvider` trait. Simple lookups remain here; enqueue and prefetch algorithms delegate to their same-name owners. / Implement trait `DependencyProvider` dùng chung của resolver. Lookup đơn giản nằm tại đây; thuật toán enqueue/prefetch chuyển tới owner cùng tên.

## Responsibility

Implements the shared resolver `DependencyProvider` trait. Simple lookups remain here; enqueue and prefetch algorithms delegate to their same-name owners. Trait signatures and the `NpmDependencyProvider` implementation route remain unchanged.

## Scope and boundary

This owner contains only this behavior. `mod.rs` wires the same-name implementation and its nearest tests. It uses shared resolver/package types and keeps the established provider API and errors.

## Phạm vi và ranh giới

Implement trait `DependencyProvider` dùng chung của resolver. Lookup đơn giản nằm tại đây; thuật toán enqueue/prefetch chuyển tới owner cùng tên. Giữ nguyên signature trait và route implementation `NpmDependencyProvider`.

Owner này chỉ chứa hành vi trên. `mod.rs` nối implementation cùng tên với test gần nhất. Owner dùng type resolver/package chung và giữ nguyên API provider cùng error contract.
