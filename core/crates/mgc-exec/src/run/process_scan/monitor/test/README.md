# Process monitor tests / Test monitor process

EN: `process_scan/` covers root visibility, process-group exit, zombie traversal and PID reuse; `process_table/` covers inspector output and row selection; `npm_probe/` covers the exact `--version` exception; `shim_security/` covers tampering and invalid Unix path bytes. Shared fixture creation and inspector parsing live under `helpers/`. Test bodies were moved from the previous monolithic file and have not been executed in this structural phase.

VI: `process_scan/` bao phủ root visibility, process group sau khi root thoát, duyệt zombie và PID reuse; `process_table/` bao phủ output inspector và chọn row; `npm_probe/` bao phủ ngoại lệ `--version` chính xác; `shim_security/` bao phủ sửa shim và byte path Unix không hợp lệ. Tạo fixture và parse inspector dùng chung nằm trong `helpers/`. Thân test được chuyển từ file monolith cũ và chưa chạy trong giai đoạn cấu trúc này.
