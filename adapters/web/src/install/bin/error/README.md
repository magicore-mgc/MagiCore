# `.bin` error mapping / Ánh xạ lỗi `.bin`

`BinError` identifies directory, manifest, link and executable failure ownership while wrapping the originating `MgError`. Converting back unwraps the same original variant; it does not rewrite public messages or error categories.

`BinError` phân loại lỗi thuộc directory, manifest, link và executable, đồng thời bọc `MgError` gốc. Khi chuyển ngược, owner trả lại đúng variant gốc; không viết lại public message/category.

This error is private to `install::bin`; shared error taxonomy remains owned by `mgc-types`. Source-only batch added no direct error tests. Test variant preservation and source behavior in the later test phase; no runtime command ran here.

Error này private trong `install::bin`; taxonomy dùng chung vẫn do `mgc-types` sở hữu. Batch source-only chưa thêm test trực tiếp cho error. Phase test sau cần kiểm variant và source; batch này chưa chạy lệnh runtime.
