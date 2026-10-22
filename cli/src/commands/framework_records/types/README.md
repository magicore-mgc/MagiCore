# Framework record types / Type record framework

## Responsibility / Trách nhiệm

Owns the unchanged public route status and record fields plus shared evidence constants. FrameworkRecord is copyable only so const per-core slices can reassemble the existing public slice.

Sở hữu status route, field record public giữ nguyên và evidence constants dùng chung. FrameworkRecord có thể copy để ghép slice const của các core thành slice public hiện tại.

## Errors and tests / Lỗi và test

Static DTO/data have no error path. Existing catalog tests stay in the CLI test owner until source decomposition closes.

DTO/dữ liệu tĩnh không có nhánh lỗi. Test catalog hiện tại giữ ở CLI test owner tới khi đóng source pass.
