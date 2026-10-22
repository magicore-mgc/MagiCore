# Import types

Owns public data types and supported legacy lockfile filename constants. These values describe input and report outcomes; they do not select or parse files.

Sở hữu kiểu dữ liệu công khai và hằng tên lockfile legacy được hỗ trợ. Các giá trị này mô tả đầu vào và kết quả báo cáo, không tự chọn hay phân tích file.

The stable paths are re-exported by `import/mod.rs`; existing callers may keep using `mgc_lockfile::import::LegacyLockfile`, `ImportReport`, `SkippedRecord`, and the format constants.

`import/mod.rs` tiếp tục export các đường dẫn ổn định; caller hiện tại vẫn dùng được `mgc_lockfile::import::LegacyLockfile`, `ImportReport`, `SkippedRecord` và hằng format.
