# Request path gate / Cổng request path

Owns the fail-closed relative-path component validator that runs before any project filesystem join.

Sở hữu bộ kiểm tra component path tương đối fail-closed, chạy trước mọi phép nối filesystem project.

Keep its behavior separate from URL decoding and filesystem routing; Axum's decoded route value is the input contract. Existing direct unit and real-server E2E checks are left untouched.

Giữ logic này tách khỏi decode URL và routing filesystem; input là giá trị route Axum đã decode. Unit trực tiếp và E2E server thật được giữ nguyên.
