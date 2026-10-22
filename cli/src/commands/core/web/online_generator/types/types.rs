//! Shared data contracts for online initializer catalogs.
//! Contract dữ liệu dùng chung cho catalog initializer online.

/// A single argument slot in an online generator invocation.
/// Một vị trí tham số trong lệnh gọi generator online.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OnlineGeneratorArgument {
    /// The user-supplied project name, placed at this position in the arg list.
    /// Tên project do user cung cấp, đặt tại vị trí này trong danh sách tham số.
    Project,
    /// A fixed literal flag or value passed verbatim to the generator binary.
    /// Cờ hoặc giá trị cố định truyền nguyên văn cho binary generator.
    Literal(&'static str),
}

/// Static description of one online scaffolding initializer.
/// Mô tả tĩnh của một initializer scaffold online.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OnlineWebGenerator {
    /// npm package providing the initializer (requested as `<package>@latest`).
    /// Gói cung cấp initializer (được yêu cầu dưới dạng `<package>@latest`).
    pub(crate) package: &'static str,
    /// Binary invoked from the installed package to run the initializer.
    /// Binary được gọi từ gói đã cài để chạy initializer.
    pub(crate) binary: &'static str,
    /// Stable identity tag recorded in the scaffold provenance string.
    /// Thẻ identity ổn định ghi vào chuỗi provenance của scaffold.
    pub(crate) identity: &'static str,
    /// Argument template; `Project` slots are filled with the project name.
    /// Mẫu tham số; vị trí `Project` được điền bằng tên project.
    pub(crate) args: &'static [OnlineGeneratorArgument],
}
