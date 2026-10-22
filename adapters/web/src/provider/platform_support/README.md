# Maps Rust host OS/CPU values to npm naming, applies positive/negative `os` and `cpu` rules, and recognizes the supported family of optional native package names. / Ánh xạ OS/CPU host sang tên npm, áp dụng rule `os`/`cpu` dương/âm, và nhận diện các họ package native optional đã hỗ trợ.

## Responsibility

Maps Rust host OS/CPU values to npm naming, applies positive/negative `os` and `cpu` rules, and recognizes the supported family of optional native package names. The owner does not download binaries or choose package versions.

## Scope and boundary

This owner contains only this behavior. `mod.rs` wires the same-name implementation and its nearest tests. It uses shared resolver/package types and keeps the established provider API and errors.

## Phạm vi và ranh giới

Ánh xạ OS/CPU host sang tên npm, áp dụng rule `os`/`cpu` dương/âm, và nhận diện các họ package native optional đã hỗ trợ. Owner không tải binary và không chọn version package.

Owner này chỉ chứa hành vi trên. `mod.rs` nối implementation cùng tên với test gần nhất. Owner dùng type resolver/package chung và giữ nguyên API provider cùng error contract.
