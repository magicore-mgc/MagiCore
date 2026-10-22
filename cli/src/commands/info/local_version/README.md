# Local declared version / Version khai báo trong project

Owns best-effort discovery of the nearest project's `package.json` and checks `dependencies`, `devDependencies`, `peerDependencies`, then `optionalDependencies`. The returned string is a declared range, not a resolved or installed version. Missing/invalid project files still produce `None`, matching current behavior.

## Tiếng Việt

Owner này tìm `package.json` của project gần nhất theo kiểu best-effort rồi kiểm tra lần lượt `dependencies`, `devDependencies`, `peerDependencies`, `optionalDependencies`. Chuỗi trả về là range khai báo, không phải version đã resolve hay đã cài. File project thiếu/lỗi vẫn trả `None`, giữ hành vi hiện tại.
