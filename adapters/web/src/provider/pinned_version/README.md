# Checks one explicitly selected version (dist-tag/add-latest/update). There is no fallback to another candidate: too-young or unstamped pins return the existing error unless the explicit missing-time option is enabled. / Kiểm tra một version được chọn rõ (dist-tag/add-latest/update). Không fallback candidate khác: version quá mới hoặc thiếu timestamp trả lỗi cũ, trừ khi bật tùy chọn cho phép thiếu thời gian.

## Responsibility

Checks one explicitly selected version (dist-tag/add-latest/update). There is no fallback to another candidate: too-young or unstamped pins return the existing error unless the explicit missing-time option is enabled. Public function and exact user-facing diagnostics remain at `provider::check_pinned_version`.

## Scope and boundary

This owner contains only this behavior. `mod.rs` wires the same-name implementation and its nearest tests. It uses shared resolver/package types and keeps the established provider API and errors.

## Phạm vi và ranh giới

Kiểm tra một version được chọn rõ (dist-tag/add-latest/update). Không fallback candidate khác: version quá mới hoặc thiếu timestamp trả lỗi cũ, trừ khi bật tùy chọn cho phép thiếu thời gian. Giữ hàm public và diagnostic tại `provider::check_pinned_version`.

Owner này chỉ chứa hành vi trên. `mod.rs` nối implementation cùng tên với test gần nhất. Owner dùng type resolver/package chung và giữ nguyên API provider cùng error contract.
