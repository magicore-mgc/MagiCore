# Bounded reader / Bộ đọc có giới hạn

Owns regular-file checks, OS no-follow/reparse flags, the lock/signature byte caps, and the stable `read_*` exports. Local typed errors map to the existing `LockfileError::IoError` and `ParseError` boundaries with the same diagnostics.

Owner này kiểm tra file thường, flag no-follow/reparse theo OS, giới hạn byte lock/chữ ký và các export `read_*` ổn định. Lỗi typed cục bộ ánh xạ về `LockfileError::IoError` và `ParseError` hiện tại với message cũ.
