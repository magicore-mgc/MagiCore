# Safe filesystem

This private CLI owner centralizes path-based directory validation and logical payload measurement for filesystem maintenance commands. It checks directory entries without following symlinks, confines canonical children to a caller-supplied canonical parent, and counts only regular-file logical lengths with checked arithmetic. It does not provide race-free directory handles; callers must retain that limitation in their safety contract.

`mod.rs` exports the narrow owner API. `root/` validates directory boundaries, `measurement/` counts payload bytes, `error.rs` preserves typed filesystem sources, and `test/` keeps tests beside these owners. Public CLI or serialized report contracts belong to their command owners.

# An toàn filesystem

Owner CLI nội bộ này tập trung xác thực thư mục theo path và đo payload logic cho lệnh bảo trì filesystem. Nó kiểm tra entry không follow symlink, giới hạn thư mục con canonical trong parent canonical do caller cung cấp và chỉ cộng độ dài logic của file thường bằng phép toán có kiểm tra tràn. Owner này không cung cấp directory handle chống race; caller phải giữ nguyên giới hạn đó trong hợp đồng an toàn.

`mod.rs` chỉ export API hẹp. `root/` xác thực ranh giới thư mục, `measurement/` đếm payload, `error.rs` giữ typed filesystem source, còn `test/` đặt test gần owner. Hợp đồng CLI và JSON thuộc owner lệnh tương ứng.
