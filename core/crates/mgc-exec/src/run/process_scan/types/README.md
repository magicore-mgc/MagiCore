# Process scan shared types

`ProcessScanSnapshot` stores OS-neutral parent/group identity, process start identity and zombie state. OS adapters populate it and the monitor consumes it. The type is exported only inside the process-scan owner.

# Kiểu dùng chung process scan

`ProcessScanSnapshot` lưu định danh parent/group trung lập OS, thời điểm bắt đầu và trạng thái zombie. Adapter OS điền dữ liệu, monitor tiêu thụ. Type chỉ được export nội bộ trong owner process-scan.
