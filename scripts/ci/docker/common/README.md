# Shared Docker CI operations

This owner handles binary extraction, entrypoint smoke checks, the pre-existing seeded Web Node image flow, and non-root write checks shared across generated profiles.

Owner này xử lý trích binary, smoke entrypoint, luồng image Web Node fixture có sẵn và kiểm tra quyền ghi non-root dùng chung cho các profile.

Keep the installed `mgc` path, status checks, image names, test command and every Docker cleanup trap unchanged when editing these scripts. A future nearest contract test should inspect these owners rather than expect long commands to remain inline in `ci.yml`.
