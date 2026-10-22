# Age policy owner

EN: Loads Web release-age configuration from project security settings and the guarded store policy, while preserving explicit untrusted-mode behavior.

VI: Đọc release-age policy Web từ security config và store có kiểm tra, giữ nguyên hành vi cờ untrusted.

This owner does not query the registry. It preserves the configured-policy → persisted-policy → strict-environment order and the optional-Web `None` branch.

Owner này không gọi registry. Nó giữ thứ tự policy cấu hình → policy trong store → environment strict và nhánh `None` khi Web optional bị tắt.
