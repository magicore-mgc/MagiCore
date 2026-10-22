# Core lifecycle lanes / Các lane lifecycle theo core

Long multi-command product checks are grouped by the core/runtime they qualify. Their workflow step still owns the environment, timeout and place in the matrix; each script preserves its original command sequence and exit behavior.

Các bài kiểm tra sản phẩm nhiều lệnh được gom theo core/runtime. Step workflow vẫn sở hữu environment, timeout và vị trí trong ma trận; mỗi script giữ nguyên thứ tự command và cách trả exit status.
