//! Ordered optional-feature package catalog — danh mục package feature tùy chọn có thứ tự.
//! Feature grouping keeps source files reviewable without changing record order.

pub(in crate::commands::core::web::implementation) struct WebFeaturePackage {
    pub(in crate::commands::core::web::implementation) feature: &'static str,
    pub(in crate::commands::core::web::implementation) section: &'static str,
    pub(in crate::commands::core::web::implementation) package: &'static str,
}

#[path = "entries/mod.rs"]
mod entries;

pub(in crate::commands::core::web::implementation) const FEATURE_PACKAGE_GROUPS:
    &[&[WebFeaturePackage]] = &[
    entries::APPLICATION_FOUNDATION,
    entries::APPLICATION_EXTENSIONS,
    entries::SERVER_DATA,
    entries::PLATFORM_INTEGRATIONS,
];

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
