# Shared manifest detection

## Purpose

This owner centralizes the five manifest gates used by MGC's shared audit lanes. Each language owner checks only its existing root-level manifest rules and returns the existing ScanStep labels and scanner closure. The facade preserves the public mgc_audit::detect route and exports used by adapters and the polyglot planner.

## Phạm vi

Owner này tập trung năm gate manifest của các lane audit dùng chung trong MGC. Mỗi owner ngôn ngữ chỉ kiểm tra đúng rule manifest hiện có và trả label ScanStep cùng scanner closure không đổi. Facade giữ route public mgc_audit::detect và các export đang được adapter với polyglot planner sử dụng.

No detector behavior, supported manifest set, filesystem policy, error behavior, or adapter opt-in changes here. There is no local state or new error contract, so this owner introduces no types.rs or error.rs.
