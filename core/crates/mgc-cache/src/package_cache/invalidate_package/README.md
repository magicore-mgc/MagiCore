# Invalidate package / Vô hiệu hóa package

Owns tamper-response removal for v2 and safe legacy entries, under the package's write shard.
Sở hữu việc xóa entry v2 và entry cũ an toàn khi phát hiện tamper, dưới write shard của package.

Removal validates the parent stays under the cache root and removes a final symlink itself instead of traversing its target.
Thao tác xóa xác nhận parent nằm trong cache root và xóa chính symlink cuối thay vì đi theo target.
