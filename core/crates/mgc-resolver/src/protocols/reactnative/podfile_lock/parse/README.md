# Podfile.lock parser / Parser Podfile.lock

The parser tracks only top-level `PODS` entries at the current two-space indentation and checksum entries under `SPEC CHECKSUMS`. It does not select versions or fetch network data.

Parser chỉ lấy entry `PODS` cấp cao với đúng mức thụt lề hai khoảng và checksum dưới `SPEC CHECKSUMS`. Module không chọn version hay tải mạng.

Parser-specific cases live in `test/parse.rs`; public DTO/checksum contract cases remain next to the parent owner.

Ca riêng parser nằm ở `test/parse.rs`; ca contract DTO/checksum công khai được đặt cạnh owner cha.
