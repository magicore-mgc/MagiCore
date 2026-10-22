# Deno lockfile parser

Parses Deno npm and JSR maps; records malformed or unsupported entries explicitly, preserves JSR provenance in names, and rebuilds workspace root pins as lockfile root dependencies.

Parse map npm và JSR của Deno; ghi rõ entry lỗi/không hỗ trợ, giữ nguồn JSR trong tên và dựng pin workspace thành root dependency của lockfile.

Import remains best-effort with an explicit skipped-record ledger; no npm-registry semantics are fabricated for JSR.

Import vẫn best-effort với sổ ghi rõ record bị bỏ qua; không bịa ngữ nghĩa npm registry cho JSR.
