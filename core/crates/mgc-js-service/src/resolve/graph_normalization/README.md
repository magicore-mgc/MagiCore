# Graph normalization owner / Owner chuẩn hóa graph

This owner preserves resolver-marked direct identities (including explicit overrides outside the manifest range), falls back to manifest-range root inference only for older graph inputs without direct flags, and prunes packages unreachable through regular or peer edges from the resulting roots.

Owner này giữ identity direct do resolver đánh dấu (kể cả override tường minh nằm ngoài range manifest), chỉ suy luận root theo range cho graph cũ không có cờ direct, rồi loại package không tới được qua regular hoặc peer edge từ các root đó.

Nearest tests cover retaining a unique explicit root, keeping reachable transitive versions, and removing unreachable packages. Graph order and edge traversal behavior remain unchanged.

Test gần nhất kiểm tra giữ explicit root duy nhất, giữ version bắc cầu còn reachable và bỏ package không reachable. Thứ tự graph và edge traversal được giữ.
