# framework_selection

Select positional/flag frontend names, detect backend framework flags in their existing priority order, and apply NestJS language defaults.

Chọn framework frontend từ đối số/cờ, nhận diện cờ backend theo đúng thứ tự ưu tiên hiện có và áp dụng mặc định ngôn ngữ cho NestJS.

`framework_selection.rs` contains the implementation; `mod.rs` wires/re-exports it. The nearest test owner pins explicit-vs-flag precedence, missing selection, backend priority and NestJS defaults.

`framework_selection.rs` chứa implementation; `mod.rs` chỉ nối/export. Test gần nhất khóa ưu tiên positional/cờ, trường hợp chưa chọn, độ ưu tiên backend và mặc định NestJS.
