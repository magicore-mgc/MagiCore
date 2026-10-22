# Python project parsing

`parse.rs` reads the regular `pyproject.toml`, validates the currently owned dependency-group shape, and projects base plus `dev` dependencies into the shared `Manifest`. Optional extras are retained in the TOML document but are not activated as install dependencies.

This owner does not resolve versions or create a lockfile. It delegates requirement syntax to `dependency` and lane policy to `ownership`. Its `error.rs` distinguishes TOML decoding, unsupported groups, and invalid dev entries; `MgError::Anyhow` retains the local error and TOML source chain while the function keeps its `MgResult` return type.

## Trách nhiệm

`parse.rs` đọc `pyproject.toml` dạng regular file, xác thực cấu trúc dependency group đang được quản lý, rồi đưa dependency nền cùng nhóm `dev` vào `Manifest` dùng chung. Optional extras được giữ trong TOML nhưng không trở thành dependency cài đặt.

Owner này không resolve version hay tạo lockfile. Cú pháp requirement do `dependency` xử lý; policy lane do `ownership` quyết định. `error.rs` phân biệt lỗi đọc TOML, group ngoài phạm vi, và phần tử dev sai dạng; `MgError::Anyhow` giữ local type/source chain TOML trong khi hàm vẫn trả `MgResult`.
