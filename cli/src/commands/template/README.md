# Template registry command

## Responsibility / Trách nhiệm

**English:** Stable facade for template naming, command dispatch, registry publish/fetch and the shared cache/archive lifecycle.

**Tiếng Việt:** Facade ổn định cho đặt tên template, điều phối command, publish/fetch registry và vòng đời cache/archive dùng chung.

## Public routes / API công khai

`commands::template::run`, `fetch`, `ensure_layer`, `extract_tarball_into`, cache helpers, `TemplateCmd` and its argument records are exported from mod.rs. Existing CLI dispatch, scaffold and all-core create callers keep those paths.

Các API trên tiếp tục được export từ mod.rs. Dispatcher CLI, scaffold và lệnh create của mọi core vẫn dùng các đường dẫn hiện tại.

## Structure / Cấu trúc

Each owner folder has mod.rs, a same-name implementation, an owner-local README and an error.rs adapter when fallible operations need the existing shared errors. Tests live under test/ by behavior; HTTP/archive fixtures have one support owner.

Mỗi owner có mod.rs, file triển khai cùng tên, README riêng và error.rs khi cần gom constructor lỗi dùng chung. Test được xếp dưới test/ theo hành vi; fixture HTTP/archive dùng chung có một owner riêng.

## Behavior boundaries / Ranh giới hành vi

This split preserves registry auth scope, redirect and size limits, digest verification, lock naming, symlink/reparse checks, archive staging, previous-cache preservation and rollback behavior. Error constructors and fallback classification remain unchanged.

Việc tách giữ nguyên phạm vi auth registry, redirect và giới hạn dung lượng, xác minh digest, tên lock, kiểm tra symlink/reparse, staging archive, giữ cache cũ và rollback. Constructor lỗi cùng phân loại fallback không đổi.

## Entry points / Điểm vào

- [Command dispatcher](template.rs)
- [Test facade](test/README.md)
