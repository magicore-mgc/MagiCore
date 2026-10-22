# Credential Debug redaction

EN: Owns the Debug representation and masks token/password values.
VI: Quản lý biểu diễn Debug và che giá trị token/password.

Boundary: Username remains visible as before; secret values remain represented only by the redaction marker.
Tests: test/debug.rs verifies secrets are absent from formatted output.
