# Task concurrency

Owns the pipeline task-limit calculation and uses the shared bounded parser so environment values cannot request a Tokio semaphore limit above the runtime maximum. The default remains the saturating sum of download and extraction limits.

Quản lý tính giới hạn task pipeline và dùng parser có giới hạn chung để env không thể yêu cầu semaphore vượt mức runtime. Mặc định vẫn là tổng saturating của download và extraction.
