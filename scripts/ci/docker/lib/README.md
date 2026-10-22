# Library language container lanes

This owner separates the Rust library lane from the Go/Python/TypeScript language loop. Each project is created and installed through MGC, checked for its core/lock/Dockerfile/Compose contract, and built/run using the named image.

Owner này tách lane thư viện Rust khỏi vòng lặp ngôn ngữ Go/Python/TypeScript. Mỗi dự án được tạo/cài qua MGC, kiểm tra core/lock/Dockerfile/Compose rồi build/chạy bằng image được đặt tên.

Keep language and image identity coupled in the polyglot loop and preserve the per-language persistent MGC data volume. Do not collapse framework-specific failures into a shared success result.
