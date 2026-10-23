# Windows path safety

## Purpose

Owns the Windows-only path checks for cached files: resolving a handle's final DOS path, proving the opened path stays under the package root, and rejecting reparse points (symlinks/junctions) along the cached relative path.

## Vietnamese

Sở hữu các kiểm tra path chỉ dành cho Windows với file đã cache: resolve final DOS path của handle, chứng minh path đã mở nằm trong root package, và từ chối reparse point (symlink/junction) trên đường dẫn tương đối.
