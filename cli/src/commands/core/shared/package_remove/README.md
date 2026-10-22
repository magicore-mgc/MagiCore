# Shared package remove

Coordinates the stable cross-core remove entrypoint and delegates request validation, dependency removal, journaled manifest commit, and optional reinstall/recovery to private owners.

## Tiếng Việt

Điều phối entrypoint remove dùng chung giữa các core và giao việc kiểm tra request, gỡ dependency, commit manifest có journal cùng reinstall/recovery tùy chọn cho các owner private.

## Phase owners

- `validation/` applies the existing package-count limit before the native-manifest owner check.
- `dependency_removal/` validates names and removes manifest entries in input order.
- `manifest_commit/` stages the journal, writes the manifest and records the post-image.
- `post_remove_install/` handles no-op/deferred paths, the existing delay hook, the disabled graph path, install and rollback.

The shared error factories remain authoritative; no local error variant or new policy is introduced.
