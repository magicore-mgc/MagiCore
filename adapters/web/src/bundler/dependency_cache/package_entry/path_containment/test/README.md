# Package path containment tests

The nearest tests verify missing outside targets are rejected lexically before filesystem resolution. Manifest and symlink scenarios are tested by their owning siblings.

Test gần owner xác minh path ra ngoài bị chặn bằng lexical check kể cả khi target chưa tồn tại. Case manifest và symlink được test cạnh owner tương ứng.
