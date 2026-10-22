# Cancellation

Owns shutdown and supervisor-cancel handling before each wait-loop inspection pass.

Cancellation preserves the sequence: terminate the process tree, release the Windows Job guard, attempt direct-child kill, bounded-drain output, then return the existing shutdown/cancellation trigger wrapped with termination failures when present.

# Hủy phiên

> Owner này xử lý shutdown và cờ cancel trước mỗi lượt kiểm tra của wait loop.

> Thứ tự hiện tại được giữ nguyên: terminate cây process, thả Windows Job guard, thử kill child, drain output có deadline, rồi trả trigger shutdown/cancel cùng các lỗi termination nếu có.
