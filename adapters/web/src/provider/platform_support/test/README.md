# Maps Rust host OS/CPU values to npm naming, applies positive/negative `os` and `cpu` rules, and recognizes the supported family of optional native package names. tests / Test Ánh xạ OS/CPU host sang tên npm, áp dụng rule `os`/`cpu` dương/âm, và nhận diện các họ package native optional đã hỗ trợ.

Focused cases: Native package family mapping for the active OS/CPU.

Case tập trung: Ánh xạ họ package native theo OS/CPU hiện tại.

`mod.rs` registers `platform_support.rs` beside the implementation. These source tests are authored/relocated; execution remains deferred until the repository source pass and SSD gate are complete.

`mod.rs` đăng ký `platform_support.rs` cạnh implementation. Test đã được viết/chuyển về; chỉ chạy sau khi kết thúc source pass toàn repo và đạt giới hạn SSD.
