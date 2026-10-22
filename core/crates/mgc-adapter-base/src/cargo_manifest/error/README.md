# Cargo manifest errors

Owns typed parse, structural-validation and serialization errors. TOML causes remain available through `std::error::Error::source`; unsupported resolver semantics retain the shared typed `MgError::Unsupported` contract.

Sở hữu lỗi typed khi parse, kiểm tra cấu trúc và serialize. Nguyên nhân TOML được giữ qua `std::error::Error::source`; semantics resolver chưa hỗ trợ vẫn dùng contract `MgError::Unsupported` chung.
