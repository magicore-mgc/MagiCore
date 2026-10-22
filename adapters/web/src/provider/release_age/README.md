# Public `AgePolicy` and the crate-visible `versions_with_age_gate` entry route every provider version list through the age filter. / Định nghĩa `AgePolicy` public và entry `versions_with_age_gate` trong crate để mọi version list của provider đi qua bộ lọc tuổi.

## Responsibility

Public `AgePolicy` and the crate-visible `versions_with_age_gate` entry route every provider version list through the age filter. Keeps the original public type and crate visibility; filtering details live in `eligible_versions`.

## Scope and boundary

This owner contains only this behavior. `mod.rs` wires the same-name implementation and its nearest tests. It uses shared resolver/package types and keeps the established provider API and errors.

## Phạm vi và ranh giới

Định nghĩa `AgePolicy` public và entry `versions_with_age_gate` trong crate để mọi version list của provider đi qua bộ lọc tuổi. Giữ public type và visibility crate như cũ; logic lọc nằm trong `eligible_versions`.

Owner này chỉ chứa hành vi trên. `mod.rs` nối implementation cùng tên với test gần nhất. Owner dùng type resolver/package chung và giữ nguyên API provider cùng error contract.
