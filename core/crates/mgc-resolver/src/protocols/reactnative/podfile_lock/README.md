# Podfile.lock owner / Owner Podfile.lock

This owner contains the public lock DTOs, checksum lookup, root-pod normalization, and the `PODS`/`SPEC CHECKSUMS` parser. Local/path pods remain excluded from registry artifacts as in the baseline.

Owner này chứa DTO lock công khai, tra checksum, chuẩn hóa tên pod gốc và parser `PODS`/`SPEC CHECKSUMS`. Pod đường dẫn/cục bộ tiếp tục bị loại khỏi artifact registry như baseline.

`parse/` owns line/section interpretation; the parent owns stable value and helper exports. Existing parser assertions are retained in the owner test folder.

`parse/` sở hữu diễn giải dòng/section; owner cha giữ export giá trị và helper ổn định. Assertion parser cũ được giữ trong folder test của owner.
