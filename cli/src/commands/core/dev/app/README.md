# App development command / Lệnh dev App

## Responsibility / Trách nhiệm

Stable `core::dev::app` API. Coordinates preflight, target/runtime selection and language dispatch; OS probes, commands and execution are separate owners.

## Tests / Kiểm thử

Owner tests are registered under the exact platform, device, command or configuration behavior. Runtime checks are deferred by the storage cap.

## Tiếng Việt

Folder này giữ API `core::dev::app`, điều phối preflight, chọn target/runtime và dispatch ngôn ngữ. Kiểm tra OS, tạo lệnh và thực thi được tách thành owner riêng. Test được đăng ký gần behavior; kiểm tra runtime đang hoãn do giới hạn dung lượng SSD.
