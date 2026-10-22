# Cargo dependency specification owner

EN: Enforces the supported dependency-table keys and checks that each declared package is locked.

VI: Chặn key dependency ngoài phạm vi và kiểm tra package khai báo đã có trong lock.

Root identity matching remains in `root_resolution/`; this owner handles only each manifest dependency specification and lock-name membership.

Đối chiếu root identity vẫn thuộc `root_resolution/`; owner này chỉ xử lý từng dependency spec và việc package có tên tương ứng trong lock.
