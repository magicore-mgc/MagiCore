# Cache runtime policy

This owner reads cache-related environment settings and wall-clock time. Download, extraction, and pipeline task concurrency are parsed as positive values no greater than the Web concurrency ceiling of 64 or Tokio's semaphore maximum, whichever is lower; invalid, zero, malformed, or over-limit values fall back to bounded defaults.

Owner này đọc cấu hình môi trường liên quan cache và thời gian hệ thống. Concurrency download, extraction và task pipeline được parse thành số dương không vượt mức Web 64 hoặc giới hạn semaphore Tokio, lấy mức thấp hơn; giá trị sai, zero, lỗi định dạng hoặc vượt giới hạn dùng default đã chặn.
