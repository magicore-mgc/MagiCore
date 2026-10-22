# JSON audit envelope

## Purpose

Owns the public borrowed envelope DTO, schema version and pretty JSON serializer while preserving the original API and serde field order.

## Phạm vi

Sở hữu DTO envelope public mượn dữ liệu, version schema và serializer JSON đẹp; giữ API gốc và thứ tự field serde.

Serialization continues to return `serde_json::Result<String>`. This owner adds no wrapper error because callers rely on the existing serde error contract.
