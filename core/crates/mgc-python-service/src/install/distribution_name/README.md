# Distribution name

`distribution_name.rs` owns PEP 503 normalization for manifest-to-lock name comparisons only. It is a pure transformation and does not produce filesystem paths or rewrite the package identity stored in metadata. The nearest test covers mixed case, repeated separators and ignored edge separators.

## Tên distribution

`distribution_name.rs` chỉ chuẩn hóa PEP 503 để đối chiếu tên trong manifest và lock. Đây là phép biến đổi thuần, không tạo path filesystem hay sửa identity package trong metadata. Test gần owner kiểm tra chữ hoa/thường, separator lặp và separator ở đầu/cuối.
