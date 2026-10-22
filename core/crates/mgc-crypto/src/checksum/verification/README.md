# Checksum verification / Xác minh checksum

This owner compares a calculated Adler-32 value with the caller's expected value. It preserves the public `anyhow::Result<bool>` contract, including its currently infallible behavior.

Owner này so sánh giá trị Adler-32 vừa tính với giá trị caller mong đợi. Nó giữ contract public `anyhow::Result<bool>`, kể cả hành vi hiện tại không phát sinh lỗi.
