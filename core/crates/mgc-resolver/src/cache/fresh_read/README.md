# Fresh cache reads / Đọc cache còn hạn

Owns the read path for version and dependency entries. An expired entry is removed only if the map still points to the exact entry observed by that read, preventing a stale reader from deleting a fresh concurrent replacement.

Sở hữu luồng đọc entry version và dependency. Entry hết hạn chỉ bị xóa nếu map vẫn trỏ đúng entry mà lần đọc đó quan sát được, tránh stale reader xóa nhầm replacement mới.
