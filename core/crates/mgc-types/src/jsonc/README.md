# Shared JSONC cleanup

Owns the public `strip_jsonc` API shared by Bun lockfile audit and import. The coordinator preserves the crate's line-normalizing output contract, while comment scanning and trailing-comma removal remain separate behavior owners. This module normalizes syntax only; strict JSON parsing remains the consumer's validation boundary.

Sở hữu API `strip_jsonc` dùng chung cho audit và import lockfile Bun. Coordinator giữ hợp đồng output chuẩn hóa dòng; quét comment và bỏ dấu phẩy cuối là hai owner hành vi riêng. Module chỉ chuẩn hóa cú pháp; consumer vẫn dùng JSON strict để xác thực.
