# Shared package update

This owner keeps the cross-core update entrypoints and coordinates manifest target selection, serial native resolution, journaled manifest commit, and optional install/recovery under the existing writer lock.

## Tiếng Việt

Owner này giữ các entrypoint update dùng chung giữa core và điều phối chọn target manifest, resolve native tuần tự, ghi manifest có journal và install/recovery tùy chọn dưới writer lock hiện có.

## Phase owners

- `target_selection/` maps all or named dependencies and retains the current unknown-package error.
- `package_resolution/` resolves in request order and mutates only the in-memory manifest.
- `manifest_commit/` stages the existing journal, writes the manifest, and records its post-image.
- `post_update_install/` reports changes, performs optional install, and finishes or rolls back the journal.
- `types/` owns the internal target data passed between selection and resolution.

The existing shared `crate::error` factories and error source chains remain authoritative; no new local error type is needed for this structural split.
