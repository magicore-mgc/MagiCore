# Parses npm registry `time` values into Unix seconds. Incomplete, impossible, pre-epoch or unrepresentable dates return `None`; this owner does not decide whether missing time is allowed. / Parse trường `time` của npm registry thành Unix seconds. Chuỗi thiếu, ngày bất khả thi, pre-epoch hoặc vượt miền `u64` trả `None`; owner này không tự quyết định có cho thiếu timestamp hay không.

## Responsibility

Parses npm registry `time` values into Unix seconds. Incomplete, impossible, pre-epoch or unrepresentable dates return `None`; this owner does not decide whether missing time is allowed. The age-policy owners apply fail-closed/escape-hatch policy to this parse result.

## Scope and boundary

This owner contains only this behavior. `mod.rs` wires the same-name implementation and its nearest tests. It uses shared resolver/package types and keeps the established provider API and errors.

## Phạm vi và ranh giới

Parse trường `time` của npm registry thành Unix seconds. Chuỗi thiếu, ngày bất khả thi, pre-epoch hoặc vượt miền `u64` trả `None`; owner này không tự quyết định có cho thiếu timestamp hay không. Các owner age policy áp dụng fail-closed/escape hatch trên kết quả parse.

Owner này chỉ chứa hành vi trên. `mod.rs` nối implementation cùng tên với test gần nhất. Owner dùng type resolver/package chung và giữ nguyên API provider cùng error contract.
