# Process-tree tracking identity

This folder owns the environment key and process-local token generator shared by spawn and descendant monitoring. The token combines the process ID, atomic sequence and current timestamp; it is an internal correlation marker, not an authentication credential. `mod.rs` exports the run-local boundary, `tracking.rs` implements the unchanged format, and `test/` pins uniqueness for successive calls. Unix-only wiring is gated by the parent owner.

# Identity theo dõi cây process

Folder này sở hữu tên biến môi trường và bộ sinh token trong process được spawn và monitor descendant dùng chung. Token gồm process ID, sequence atomic và timestamp hiện tại; đây là marker đối chiếu nội bộ, không phải credential xác thực. `mod.rs` export biên nội bộ của run, `tracking.rs` giữ format hiện tại, `test/` khóa tính duy nhất giữa các lần gọi liên tiếp. Wiring chỉ dành Unix được gate tại owner cha.
