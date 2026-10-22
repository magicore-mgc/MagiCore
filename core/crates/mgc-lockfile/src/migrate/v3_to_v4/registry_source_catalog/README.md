# Registry source catalog / Catalog nguồn registry

Builds one v4 `SourceRef` per distinct registry URL, preserves encounter order, and labels unattributed packages with the explicit `unknown` source plus a warning. It owns the stable registry-derived source ID helper.

Dựng một `SourceRef` v4 cho mỗi URL registry khác nhau, giữ thứ tự gặp đầu tiên và gắn nhãn package không rõ nguồn bằng nguồn `unknown` cùng warning. Folder này sở hữu helper tạo ID ổn định từ registry.

`mod.rs` wires the implementation and nearest-owner tests. The test verifies deduplication and unknown attribution; it has not been executed yet.

`mod.rs` nối implementation và test gần owner. Test kiểm tra gộp registry và gán nguồn unknown; hiện chưa chạy.
