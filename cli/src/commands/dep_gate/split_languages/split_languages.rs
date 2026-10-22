//! Split-language capability inventory — danh sách capability theo language.
//! Names emitted for cells whose ownership differs by ecosystem.
//! Tên được xuất khi ownership của cell khác theo ecosystem.

/// Languages with a potential per-language ownership split, enumerated by
/// `capabilities --json` so the matrix cross-check sees the same table
/// the firewall enforces (only differing languages are emitted).
/// "javascript" is included because the lifecycle matrix taxonomy names
/// the web lane's language that way (gate id "js" is the lane-passed id);
/// "bevy" / the iot framework ids / "terraform" are the delegated lanes'
/// gate ecosystems (the matrix cross-check resolves (game,rust) → bevy
/// and (iot,rust) → esp32-rust through them).
/// Các id `javascript`, `bevy`, framework IoT và `terraform` giữ đúng taxonomy
/// lifecycle matrix; chúng không tự mở rộng owner hay quyền chạy dependency.
/// (Ngôn ngữ có thể tách sở hữu riêng, để JSON capabilities thấy đúng
/// bảng tường lửa cưỡng chế — chỉ emit ngôn ngữ khác biệt.)
pub const SPLIT_LANGUAGES: &[&str] = &[
    "ts",
    "js",
    "rust",
    "python",
    "go",
    "java",
    "dotnet",
    "flutter",
    "swift",
    "kotlin",
    "objc",
    "rn",
    "javascript",
    "bevy",
    "esp32-rust",
    "platformio",
    "zephyr",
    "terraform",
];
