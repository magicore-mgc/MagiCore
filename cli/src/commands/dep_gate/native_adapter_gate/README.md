# Native adapter gate

Checks adapter identity, the AI manifest exception, static owner, declared operation capability and native update support in that order. Identity match never bypasses the ownership table.

Giữ nguyên thứ tự fail-closed và diagnostic hiện có. `test/` kiểm tra identity mismatch, capability bắt buộc, hỗ trợ update và từ chối compat qua adapter stub tối thiểu.
