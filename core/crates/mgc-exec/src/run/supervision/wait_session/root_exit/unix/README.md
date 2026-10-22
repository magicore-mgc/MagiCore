# Unix root exit

Owns the last process-tree checks after the root child has been reaped.

The root visibility exception is tolerated only as in the previous implementation. Other inspection failures terminate the tree and retain their source; process-group survivors are terminated and reported; the output deadline remains tied to the original execution start.

# Unix root-exit

> Owner này thực hiện các kiểm tra process cuối sau khi root child được reap.

> Ngoại lệ root không hiển thị được xử lý đúng như trước; lỗi inspection khác sẽ terminate cây và giữ source; process group còn descendant sẽ bị dừng và báo lỗi; deadline output vẫn tính từ lúc execution bắt đầu.
