# Unix shell path owner

`mod.rs` only wires the platform owners and exports the Unix entry point. `unix.rs` selects Bash/Zsh/Fish startup files. Named child owners validate and update profiles, render the managed marker, create Fish config directories safely, and replace files atomically. Marker unit tests stay beside their owner; public API integration cases stay in the crate integration suite. Native OS behavior still requires CI.

## Vietnamese

`mod.rs` chỉ wiring các owner của nền tảng và export entry Unix. `unix.rs` chọn startup file Bash/Zsh/Fish. Các owner con kiểm tra/cập nhật profile, render marker, tạo thư mục Fish an toàn và thay file nguyên tử. Unit test marker nằm cạnh owner; integration test API public ở cấp crate. Hành vi OS native vẫn cần CI.
