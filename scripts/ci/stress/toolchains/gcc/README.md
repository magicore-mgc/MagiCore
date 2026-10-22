# Hosted Windows GCC toolchain / GCC trên Windows runner

`expose_windows.ps1` discovers a complete GCC/G++ pair on the hosted runner, adds its directory to the workflow PATH, and prints both compiler versions. The workflow runs it only on Windows.

`expose_windows.ps1` tìm đủ cặp GCC/G++ trên hosted runner, thêm thư mục vào PATH workflow và in version của cả hai compiler. Workflow chỉ chạy script trên Windows.
