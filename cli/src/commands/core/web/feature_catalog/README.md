# Web optional-feature package catalog

`feature_catalog.rs` owns the record type and ordered group list. `entries/` contains four contiguous data-only groups. S408 removed 14 later exact duplicate tuples, retaining first occurrence order; the nearest hash test covers the resulting 79 unique tuples. Repeated package names under different feature IDs are not collapsed and may still cause more than one resolution when both flags are active.

`feature_catalog.rs` sở hữu type bản ghi và danh sách nhóm có thứ tự. `entries/` chứa bốn nhóm dữ liệu liên tiếp. S408 xóa 14 tuple trùng hoàn toàn phía sau, giữ thứ tự lần xuất hiện đầu; test hash gần owner khóa 79 tuple duy nhất sau xử lý. Package lặp dưới feature ID khác nhau không bị gộp và vẫn có thể phân giải nhiều lần nếu bật cả hai flag.
