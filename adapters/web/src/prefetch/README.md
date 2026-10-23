# Web tarball prefetch

## Purpose

Owns `spawn_tarball_download`: warms the shared cache after resolve using the bounded download concurrency and the tarball prefetch lock. Returns a join handle carrying the downloaded byte count.

## Phạm vi

Sở hữu `spawn_tarball_download`: làm ấm cache chung sau resolve với giới hạn concurrency tải và lock prefetch tarball. Trả join handle kèm số byte đã tải.
