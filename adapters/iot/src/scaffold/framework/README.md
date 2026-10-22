# IoT framework file generators

This facade dispatches to one output owner per supported IoT framework. Each child owns only the starter files for that framework; the shared toml_string owner safely renders user-controlled TOML string values. The ESP32 Rust lane also validates Cargo package names before filesystem writes. Board validation and the public scaffold API remain in the coordinator.

Facade này chuyển tới owner output riêng cho mỗi framework IoT được hỗ trợ. Mỗi nhánh chỉ sở hữu file khởi tạo của framework đó; owner toml_string serialize an toàn giá trị TOML do user cung cấp. Lane ESP32 Rust còn xác thực tên Cargo trước khi ghi filesystem. Xác thực board và API scaffold public thuộc coordinator.
