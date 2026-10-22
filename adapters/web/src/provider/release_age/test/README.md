# Public `AgePolicy` and the crate-visible `versions_with_age_gate` entry route every provider version list through the age filter. tests / Test Định nghĩa `AgePolicy` public và entry `versions_with_age_gate` trong crate để mọi version list của provider đi qua bộ lọc tuổi.

Focused cases: Age-filter behavior across unstamped, young and old registry entries.

Case tập trung: Hành vi age filter với metadata thiếu timestamp, version mới và cũ.

`mod.rs` registers `release_age.rs` beside the implementation. These source tests are authored/relocated; execution remains deferred until the repository source pass and SSD gate are complete.

`mod.rs` đăng ký `release_age.rs` cạnh implementation. Test đã được viết/chuyển về; chỉ chạy sau khi kết thúc source pass toàn repo và đạt giới hạn SSD.
