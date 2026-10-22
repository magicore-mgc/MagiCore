# Project install lock handle

Owns the lock-file representation, canonical lock path, try-lock acquisition, and release-on-drop behavior.

## Trách nhiệm

Quản lý representation file lock, đường dẫn chuẩn hóa, thao tác try-lock và nhả khóa khi drop.

The parent module re-exports `ProjectInstallLock` and keeps the `Database` constructor. Preserve fail-fast acquisition and leave the zero-byte lock file in place after drop.

Module cha re-export `ProjectInstallLock` và giữ constructor `Database`. Giữ try-lock fail-fast và để nguyên file lock 0-byte sau drop.
