# process_invocation

Owns conversion from a runtime script operand to the manager identity used by process policy. It relies on the shared allowlist token normalization and removes shell script suffixes as before.

Sở hữu việc đổi script operand của runtime thành identity manager cho process policy. Module dùng chuẩn hóa token chung của allowlist và bỏ hậu tố shell như hành vi hiện tại.

This is string-only logic and returns a name rather than a fallible result, so no local error type is needed. The nearest test is in test/process_invocation.rs.
