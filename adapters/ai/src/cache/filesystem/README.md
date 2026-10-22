# AI cache filesystem

Shared filesystem owners for AI cache measurement and destructive operations live here. Recursive size uses `symlink_metadata`/`DirEntry::file_type`, skips links and special entries, and propagates I/O errors instead of following a link cycle or reporting a partial total.

Owner filesystem dùng chung cho đo dung lượng và thao tác xóa cache AI nằm tại đây. Đo size đệ quy dùng `symlink_metadata`/`DirEntry::file_type`, bỏ symlink và entry đặc biệt, propagate lỗi I/O thay vì đi theo vòng link hoặc báo tổng thiếu.

The logical-byte result is not physical disk reclamation; hard links, sparse files and concurrent mutation can change actual storage recovery. The `logical_size/` owner has nearest source cases.

Logical-byte result không đồng nghĩa dung lượng vật lý được thu hồi; hard link, sparse file và thay đổi đồng thời có thể làm thay đổi dung lượng thật. Owner `logical_size/` có test source gần nhất.
