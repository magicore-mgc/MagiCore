# Atomic replacement / Thay file atomic

This owner preserves the `mgc_lockfile::atomic::atomic_replace_file` API by re-exporting the shared `mgc-platform` primitive. The lockfile crate no longer owns a duplicate OS implementation.

Owner này giữ API `mgc_lockfile::atomic::atomic_replace_file` bằng cách re-export primitive dùng chung từ `mgc-platform`. Crate lockfile không còn sở hữu implementation OS trùng lặp.

It does not create staging data, acquire locks, or decide whether a lock may be mutated. Callers retain those responsibilities.

Module không tạo staging data, acquire lock hoặc quyết định có được sửa lock hay không. Các trách nhiệm đó thuộc caller.
