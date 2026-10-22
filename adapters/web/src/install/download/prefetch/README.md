# Tarball prefetch

Owns cache lookup, integrity verification, scoped registry downloads, bounded download permits, and publication into the project/shared package cache. Semaphore and task-join failures have local typed errors. It keeps the `prefetch_tarballs` entry point stable through `install::download`.

Quản lý tra cache, xác minh integrity, tải bằng registry theo scope, permit download có giới hạn và publish vào cache project/shared. Entry point `prefetch_tarballs` vẫn ổn định qua `install::download`.
