# Package conversion / Chuyển đổi package

Projects each legacy pin into a v4 key, carries metadata and verifiable artifact evidence, and converts dependency names only when the target is unique. Missing or ambiguous edges and versionless peers produce ordered warnings instead of fabricated graph data.

Chiếu từng pin cũ thành key v4, giữ metadata/bằng chứng artifact có thể xác minh, và chỉ chuyển tên dependency khi đích là duy nhất. Cạnh thiếu/mơ hồ và peer không có version tạo warning theo thứ tự thay vì bịa dữ liệu đồ thị.

`mod.rs` wires this owner and its nearest tests. The tests cover unique, missing and ambiguous targets; they have not been executed yet.

`mod.rs` nối owner này với test gần nhất. Test bao phủ đích duy nhất, thiếu và mơ hồ; hiện chưa chạy.
