# JSON audit output

## Purpose

This owner preserves the public versioned JSON envelope path and delegates report projection, stable scanner slugs and skip-reason mapping to focused owners. The CLI remains the production consumer.

## Phạm vi

Owner này giữ đường dẫn public của envelope JSON có version và tách ánh xạ report, slug scanner ổn định cùng lý do bị bỏ qua thành các owner nhỏ. CLI tiếp tục là consumer production.

Empty `skipped_reasons` is omitted by the existing serde annotation. Output shape, schema version and the serializer error type remain unchanged.
