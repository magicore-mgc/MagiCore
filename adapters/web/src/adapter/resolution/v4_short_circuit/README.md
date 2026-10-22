# v4 lock short-circuit / Tái sử dụng lock v4

Owns the exact v4-lock read, integrity check, project-owner match and lossless graph conversion used before the regular resolver. A miss stays a miss; corrupt v4 documents retain their existing error path.

Sở hữu đọc lock v4, kiểm integrity, so owner project và chuyển graph không mất dữ liệu trước resolver thường. Miss vẫn là miss; lock v4 hỏng giữ nguyên đường báo lỗi.
