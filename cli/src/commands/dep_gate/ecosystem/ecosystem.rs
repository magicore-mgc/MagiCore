//! Canonical ecosystem identifiers — định danh ecosystem chuẩn.
//! Single vocabulary matched by ownership policy; display labels stay outside the gate.
//! Một vocabulary duy nhất để policy khớp; nhãn hiển thị không đi vào gate.

/// Canonical ecosystem ids — the ONLY strings the firewall matches on.
/// Lanes MUST map their detected language/framework to these (via the
/// adapter enums' `ecosystem()`), never pass display names through:
/// `AppLanguage::as_str()` yields "react-native" but the gate id is "rn".
/// (Id ecosystem chuẩn — chuỗi DUY NHẤT tường lửa khớp. Lane PHẢI map
/// ngôn ngữ detect được sang các id này, không bao giờ truyền tên hiển
/// thị: `as_str()` cho "react-native" nhưng id gate là "rn".)
pub mod eco {
    pub const JS: &str = "js";
    pub const TS: &str = "ts";
    pub const PYTHON: &str = "python";
    pub const RUST: &str = "rust";
    pub const GO: &str = "go";
    pub const JAVA: &str = "java";
    pub const DOTNET: &str = "dotnet";
    pub const FLUTTER: &str = "flutter";
    pub const KOTLIN: &str = "kotlin";
    pub const SWIFT: &str = "swift";
    pub const OBJC: &str = "objc";
    /// React Native gate id (NOT "react-native").
    /// ID gate React Native là `rn`, không phải nhãn `react-native`.
    pub const RN: &str = "rn";
    // NOTE: no MULTI const — a multi-platform project never gates as a
    // unit (install_multi gates per platform with android/ios/flutter/
    // react-native ids); passing "multi" falls to Unsupported by design.
    // Project multi không được gate như một nhóm; mỗi nền tảng phải có context riêng.
    pub const BEVY: &str = "bevy";
    pub const TERRAFORM: &str = "terraform";
}
