# File Identity Owner

**Purpose:** Obtain the OS-specific identity used to decide whether a prior blob verification may be reused.

**Mục đích:** Lấy identity theo OS để quyết định có thể dùng lại kết quả verify blob hay không.

Unix fingerprints use device, inode, size, mtime, and ctime and reject symlinks. Non-Unix builds return no identity so callers rehash instead of memoizing. Tests for both cfg paths live under `test/`.

Fingerprint Unix dùng device, inode, size, mtime, ctime và từ chối symlink. Ngoài Unix trả về không có identity để caller rehash thay vì memo hóa. Test cho hai nhánh cfg ở `test/`.
