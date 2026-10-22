# AI prune API types

This folder owns the stable policy enum and result shape shared by prune selection and the public cache API.

Folder này sở hữu enum policy và result shape ổn định được logic chọn prune và public cache API dùng chung.

The `KeepRecent` timestamp field and byte-count interpretation are documented on the public types; nearest construction coverage is in `test/types.rs`.

Trường timestamp của `KeepRecent` và cách hiểu byte count được ghi ở public type; test khởi tạo gần nhất nằm tại `test/types.rs`.
