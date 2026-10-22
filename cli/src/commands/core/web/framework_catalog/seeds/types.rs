// Framework seed data types — kiểu dữ liệu cho cấu hình seed framework.
// These private catalog types are shared by each framework data owner.
pub(in crate::commands::core::web::implementation) struct WebFrameworkSeed {
    pub(in crate::commands::core::web::implementation) name: &'static str,
    /// Packages that use the user's requested version.
    /// Package dùng version mà user yêu cầu.
    pub(in crate::commands::core::web::implementation) primary: &'static [&'static str],
    /// Packages resolved independently from the registry latest path.
    /// Package được phân giải độc lập theo nhánh registry latest.
    pub(in crate::commands::core::web::implementation) supplemental: &'static [&'static str],
    pub(in crate::commands::core::web::implementation) toolchain: &'static [WebToolchainPackage],
}

pub(in crate::commands::core::web::implementation) struct WebToolchainPackage {
    pub(in crate::commands::core::web::implementation) section: &'static str,
    pub(in crate::commands::core::web::implementation) package: &'static str,
    pub(in crate::commands::core::web::implementation) typescript_only: bool,
    /// If set, use this version instead of fetching from npm registry.
    /// Nếu có giá trị, dùng version này thay vì hỏi registry npm.
    pub(in crate::commands::core::web::implementation) version: Option<&'static str>,
}
