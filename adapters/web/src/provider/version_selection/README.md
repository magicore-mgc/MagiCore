# Owns stable cache-key formatting, semver filtering, stable-over-prerelease preference unless the spec explicitly requests a prerelease, and registry metadata version parsing. / Quản lý format cache key, lọc semver, ưu tiên stable hơn prerelease trừ khi spec yêu cầu prerelease, và parse version từ metadata registry.

## Responsibility

Owns stable cache-key formatting, semver filtering, stable-over-prerelease preference unless the spec explicitly requests a prerelease, and registry metadata version parsing. Invalid semver specs preserve the existing `DependencyError` text.

## Scope and boundary

This owner contains only this behavior. `mod.rs` wires the same-name implementation and its nearest tests. It uses shared resolver/package types and keeps the established provider API and errors.

## Phạm vi và ranh giới

Quản lý format cache key, lọc semver, ưu tiên stable hơn prerelease trừ khi spec yêu cầu prerelease, và parse version từ metadata registry. Spec semver sai giữ nguyên nội dung `DependencyError`.

Owner này chỉ chứa hành vi trên. `mod.rs` nối implementation cùng tên với test gần nhất. Owner dùng type resolver/package chung và giữ nguyên API provider cùng error contract.
