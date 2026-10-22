# Dependency delegation audit

This package implements the security gate that inventories process boundaries across Rust, Python, shell, JavaScript and PowerShell. The legacy executable and import surfaces stay at `scripts/audit_dependency_delegation.py`; CI continues invoking the original root paths.

The package root exports the stable API through `audit.py`. Language scanners own matching rules and scanner behavior; `runner` owns ledger assembly and verdict. File and parser failures retain the gate's existing fail-closed behavior. Python native exceptions are propagated or converted only where the existing scanner already defines that policy; this refactor adds no catch-all suppression.

## Tiếng Việt

Gói này triển khai security gate kiểm kê process boundary cho Rust, Python, shell, JavaScript và PowerShell. Đường dẫn CLI/import cũ tại `scripts/audit_dependency_delegation.py` được giữ ổn định để CI không đổi.

API tương thích nằm ở `audit.py`; scanner từng ngôn ngữ sở hữu rule và xử lý riêng, `runner` tổng hợp ledger và verdict. Lỗi đọc/parser giữ nguyên chính sách fail-closed hiện hành; không thêm bắt lỗi tổng quát để che lỗi.
