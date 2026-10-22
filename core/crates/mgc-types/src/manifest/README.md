# Shared Manifest owner

Owns the public shared `Manifest` aggregate and its dependency-group operations. The owner facade exports the type; the crate root keeps both `mgc_types::Manifest` and `mgc_types::manifest::Manifest`. The model is infallible and has no local error owner.

Sở hữu aggregate `Manifest` dùng chung và các thao tác nhóm dependency. Facade owner export kiểu; crate root giữ cả hai đường dẫn `mgc_types::Manifest` và `mgc_types::manifest::Manifest`. Model không có nhánh lỗi nên không cần error owner riêng.

The local tests cover the existing add, replace, move, remove, iteration, lookup and group-order behavior. A separate integration smoke pins both public paths. No adapter caller is copied or changed here.

Test local bao phủ hành vi add, replace, move, remove, duyệt, tra cứu và thứ tự group hiện có. Một integration smoke riêng ghim cả hai public path. Không sao chép hay sửa caller adapter ở đây.
