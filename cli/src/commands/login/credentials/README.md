# Credential input

EN: Owns username prompting and password acquisition. Passwords come either from explicitly requested stdin or the existing no-echo terminal prompt; stdin remains size-bounded.

VI: Owner này nhập username và nhận password. Password đến từ stdin khi user chọn rõ cờ tương ứng hoặc prompt terminal không echo như hiện tại; stdin vẫn có giới hạn kích thước.

Keep password bytes out of logs and diagnostics. The public crate-local `commands::login::read_password` facade retains its previous signature for registry user management. Pure input normalization tests are in `test/credentials.rs`.

Không ghi password vào log/diagnostic. Facade crate-local `commands::login::read_password` giữ signature cũ để registry user management tiếp tục dùng. Test chuẩn hóa input thuần ở `test/credentials.rs`.
