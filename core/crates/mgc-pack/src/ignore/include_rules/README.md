# Package inclusion rules

This owner defines the fixed package metadata files that are restored after ignore filtering and the repository/generated path components that are always rejected. It has no filesystem access and no independent error contract.

The `test_ignore` Cargo integration target exercises these rules through the stable public API, with cases in this owner's `test/` directory. Keep this policy owner private behind `mgc_pack::ignore::select_files`.

READMEVN: Owner này chỉ giữ luật include/exclude thuần dữ liệu và không truy cập filesystem. Các luật được kiểm chứng qua test chọn file ở owner cha; API policy vẫn private.
