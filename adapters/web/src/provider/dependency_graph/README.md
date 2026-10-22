# Parses `npm:` alias specs and turns dependencies, optional dependencies and non-optional peer dependencies into `ResolvedDep` edges. Optional peers remain compatibility hints and are not auto-fetched. / Parse alias `npm:` và chuyển dependencies, optional dependencies, peer không optional thành edge `ResolvedDep`. Optional peer vẫn chỉ là gợi ý tương thích, không tự tải.

## Responsibility

Parses `npm:` alias specs and turns dependencies, optional dependencies and non-optional peer dependencies into `ResolvedDep` edges. Optional peers remain compatibility hints and are not auto-fetched. Uses shared `ResolvedDep` and `PackageName` types; malformed names/specs keep the existing skip behavior.

## Scope and boundary

This owner contains only this behavior. `mod.rs` wires the same-name implementation and its nearest tests. It uses shared resolver/package types and keeps the established provider API and errors.

## Phạm vi và ranh giới

Parse alias `npm:` và chuyển dependencies, optional dependencies, peer không optional thành edge `ResolvedDep`. Optional peer vẫn chỉ là gợi ý tương thích, không tự tải. Dùng type chung `ResolvedDep`/`PackageName`; tên/spec không hợp lệ giữ nguyên cách bỏ qua hiện tại.

Owner này chỉ chứa hành vi trên. `mod.rs` nối implementation cùng tên với test gần nhất. Owner dùng type resolver/package chung và giữ nguyên API provider cùng error contract.
