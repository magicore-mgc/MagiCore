# Symlink materialization

## Purpose

Owns `create_symlink`: idempotent symlink/junction replacement with read-only-aware removal, parent creation, and the Windows hardlink-tree fallback. Unix and Windows removal primitives stay `cfg`-split; the caller-facing route remains `crate::layout::create_symlink` through a thin facade.

## Phạm vi

Owner này sở hữu `create_symlink`: thay symlink/junction idempotent, xoá có xử lý read-only, tạo parent và fallback hardlink-tree trên Windows. Primitive xoá Unix/Windows vẫn tách bằng `cfg`; route cho caller vẫn là `crate::layout::create_symlink` qua facade mỏng.
