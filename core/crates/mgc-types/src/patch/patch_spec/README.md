# Patch-spec owner

Owns the configuration-time `PatchSpec` and its associated `PatchKind`. Their public fields, serde casing/defaults and constructor are kept together because they form one config contract.

Sở hữu `PatchSpec` dùng ở cấu hình và `PatchKind` đi kèm. Fields public, serde casing/default và constructor được giữ cùng nhau vì chúng tạo thành một contract cấu hình.

This module describes patch metadata; it does not fetch, validate or apply a patch file. Its nearest test covers serialization only.

Module này mô tả metadata patch; không tải, xác thực hay áp dụng file patch. Test gần owner chỉ kiểm tra serialization.
