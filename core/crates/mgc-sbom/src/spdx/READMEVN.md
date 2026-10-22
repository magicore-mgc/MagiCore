# Bộ tuần tự hóa SPDX

Facade private của crate giữ nguyên `crate::spdx::serialize`. Các owner con xác thực request, chiếu identity/checksum/relationship, lắp DTO SPDX và ánh xạ lỗi cục bộ đúng một lần tại boundary.
