# SARIF 2.1.0 output

## Purpose

This owner preserves the shared mgc-audit SARIF API and schema. Its facade exports the public log/result model and to_sarif; internal owners build driver trust metadata, vulnerability results and the final run. The CLI remains the consumer of this stable contract.

## Phạm vi

Owner này giữ API SARIF và schema dùng chung của mgc-audit. Facade export model log/result public và to_sarif; các owner nội bộ dựng metadata driver/trust, finding result và SARIF run cuối. CLI tiếp tục dùng contract ổn định này.

All vulnerability rows supplied by AuditReport are serialized, even when scanner_status is not Available; the driver is then explicitly marked UNVERIFIED. This owner does not filter or fabricate findings.
