# SARIF assembly tests

## Purpose

Checks the stable log schema and that supplied findings remain visible with an explicit UNVERIFIED driver on a non-Available report.

## Phạm vi

Kiểm tra schema log ổn định và giữ finding đã có cùng driver UNVERIFIED rõ ràng khi report không ở trạng thái Available.

The public CLI E2E remains in cli/tests/audit_cli_e2e.rs. Local unit tests do not replace that real-binary route and are NOT_RUN in this phase.
