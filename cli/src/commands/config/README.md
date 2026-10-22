# CLI config / Config CLI

## Responsibility / Trách nhiệm

Keeps the existing public command path, Clap contract and dispatch. Behavior is split into focused read, write, redaction, path and atomic-I/O owners.

The local source-progress record is in [ProgressReport.md](ProgressReport.md); it distinguishes static source evidence from the still-deferred runtime checks.

## Tiếng Việt

Giữ đường dẫn command public, contract Clap và điều phối hiện có. Hành vi được chia thành owner đọc, ghi, che bí mật, đường dẫn và I/O atomic.

Hồ sơ tiến độ source tại chỗ là [ProgressReport.md](ProgressReport.md); tài liệu tách rõ bằng chứng rà tĩnh với kiểm chứng runtime còn hoãn.
