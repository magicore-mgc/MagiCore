# AI scaffold owners

This folder keeps the public `AiProcessor` facade and separates each generated output by responsibility. The coordinator writes only project files; it does not invoke Python, a package manager, a compiler or a shell. The `run_dev.sh` file is generated output and is never executed by scaffold generation.

Thư mục này giữ facade `AiProcessor` và tách từng output theo trách nhiệm. Coordinator chỉ ghi file dự án; không gọi Python, package manager, compiler hoặc shell. `run_dev.sh` là file được sinh, không được chạy trong luồng tạo scaffold.
