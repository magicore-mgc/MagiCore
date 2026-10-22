# TOML string value owner

Serializes one untrusted string as a TOML value for all IoT framework generators. It owns value escaping only; it does not validate Cargo package-name grammar.

Serialize một chuỗi không tin cậy thành giá trị TOML dùng chung cho generator IoT. Owner chỉ xử lý escaping giá trị; không xác thực cú pháp tên package Cargo.
