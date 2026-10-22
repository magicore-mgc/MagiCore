# IoT scaffold coordinator

Validates that a board belongs to the selected framework before creating the target directory, then dispatches to the framework-specific file generator. It owns the stable `scaffold_project` API and does not install dependencies.

Coordinator scaffold IoT xác thực board thuộc framework đã chọn trước khi tạo thư mục đích, sau đó gọi generator riêng. Owner này giữ API `scaffold_project` ổn định và không cài dependency.
