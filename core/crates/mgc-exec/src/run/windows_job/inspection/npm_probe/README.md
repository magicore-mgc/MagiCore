# Windows npm probe authentication

Owns the exact MGC npm version-probe exemption for Windows: image name, argv shape and executable-byte identity must all match. `mod.rs` exports the inspection-facing functions; `npm_probe.rs` implements authentication; `error.rs` gives typed refusal reasons without exposing paths, argv or hashes. `test/mod.rs` explicitly registers native/alias and error tests; `docs/` tracks this owner. POSIX shell/shim recognition remains in the shared runner.

# Xác thực npm probe Windows

Sở hữu ngoại lệ npm version-probe chính xác của MGC trên Windows: tên image, dạng argv và danh tính byte executable đều phải khớp. `mod.rs` export hàm cho inspection; `npm_probe.rs` triển khai xác thực; `error.rs` tạo lý do từ chối typed, không lộ path/argv/hash. `test/mod.rs` đăng ký tường minh test native/alias và error; `docs/` theo dõi owner. Nhận diện shell/shim POSIX vẫn ở runner dùng chung.
