# IoT project scaffolding

The public `scaffold_project` entry validates a framework/board pair before creating the project directory. It dispatches to separate ESP32-Rust, PlatformIO or Zephyr owners that write only their current starter files. Coordinator tests cover early rejection; each framework folder owns its output test.

Scaffolding writes starter files; it does not install dependencies, invoke framework package managers, initialize Zephyr west, build firmware, or flash hardware.

Entry point `scaffold_project` kiểm tra cặp framework/board trước khi tạo thư mục project. Nó chuyển tới owner ESP32-Rust, PlatformIO hoặc Zephyr để ghi các file khởi tạo hiện có. Test coordinator kiểm tra từ chối sớm; từng folder framework sở hữu test output.

Scaffold chỉ ghi file khởi tạo; không cài dependency, gọi package manager framework, khởi tạo Zephyr west, build firmware hoặc flash phần cứng.
