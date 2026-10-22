# Dependency gate context

Owns the complete detected lane input: core, canonical ecosystem, framework, target, manifest format, and operation, plus its human-readable description. Do not construct incomplete context to get a more permissive decision.

Sở hữu input đầy đủ đã detect và chuỗi mô tả; thiếu bằng chứng phải giữ fail-closed. `test/` kiểm tra định dạng mô tả.
