# CAS Membership Owner

**Purpose:** Own CAS presence checks and file removal, including invalidating the matching memo entry.

**Mục đích:** Sở hữu kiểm tra blob tồn tại và xóa file CAS, gồm vô hiệu hóa memo tương ứng.

The public `ContentStore::contains` and `remove` signatures remain unchanged. `test/` covers state before and after removal.

Chữ ký public `ContentStore::contains` và `remove` được giữ nguyên. `test/` kiểm tra trạng thái trước và sau khi xóa.
