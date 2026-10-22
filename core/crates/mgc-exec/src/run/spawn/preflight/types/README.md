# Preflight types

`PreparedInvocation` stores only the redacted argv, resolved cwd/timeout, and compat audit context needed after preflight. Its visibility is restricted to `run::implementation`; it does not change the public executor API.

`PreparedInvocation` chỉ giữ argv đã redact, cwd/timeout đã phân giải và ngữ cảnh audit compat cần sau preflight. Visibility chỉ trong `run::implementation`; không đổi API public.
