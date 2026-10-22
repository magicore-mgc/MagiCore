# CI/CD provider owner

This owner preserves the public provider type/detector route and the crate-private manifest classifier.
Owner này giữ route public của type/detector provider và classifier manifest nội bộ crate.

Provider parsing, project config, marker priority and manifest recognition have distinct source owners. Read/parse outcomes stay best-effort where the old API returns Option or bool.
Parser provider, config project, thứ tự marker và nhận diện manifest có owner source riêng. Kết quả đọc/parse tiếp tục best-effort tại API cũ trả Option hoặc bool.
