# Lockfile migration / Migration lockfile

Owns v1/v2/v3 upgrades, explicit v3-to-v4 conversion and adapter lane merge helpers. `mod.rs` preserves `mgc_lockfile::migrate::*`; each behavior sits in a same-name owner and has a separate nearest-owner test.

Sở hữu nâng cấp v1/v2/v3, chuyển v3 sang v4 tường minh và helper merge lane cho adapter. `mod.rs` giữ `mgc_lockfile::migrate::*`; mỗi hành vi nằm trong owner cùng tên và có test gần nhất tách riêng.

Integrity is evidence about package bytes. A legacy v1 record has no digest, so migration keeps integrity empty; download/install code may compute it from the actual bytes. It must never hash the resolved URL and present that as package integrity.

Integrity là bằng chứng về bytes package. Record v1 cũ không có digest nên migration giữ integrity rỗng; luồng tải/install có thể tính từ bytes thật. Không được hash URL rồi trình bày kết quả như integrity package.
