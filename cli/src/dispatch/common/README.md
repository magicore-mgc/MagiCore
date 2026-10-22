# Shared CLI dispatch owner

Owns the common-command route table. `common.rs` keeps the single exhaustive match so every `CommonCommand` variant must be routed at compile time. The audit-signature policy and feature-gated deploy target decision live in separate owners. Existing command modules retain their implementation and error ownership. All failures continue to use shared `anyhow::Result` and shared CLI error constructors.

## Tiếng Việt

Owner sở hữu bảng route lệnh CLI dùng chung. `common.rs` giữ match exhaustive duy nhất để compiler buộc mọi variant `CommonCommand` phải có route. Policy audit signature và lựa chọn deploy theo feature có owner riêng. Command module hiện hữu tiếp tục sở hữu implementation và lỗi. Mọi lỗi dùng `anyhow::Result` và error constructor CLI dùng chung.
