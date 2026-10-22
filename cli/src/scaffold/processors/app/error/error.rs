//! Typed error for unsupported App frameworks.
//! Lỗi typed cho framework App không hỗ trợ.

#[derive(Debug, thiserror::Error)]
pub(in crate::scaffold::processors::app::implementation) enum AppScaffoldError {
    #[error(
        "unsupported app framework '{framework}'; supported values are \
         flutter, kotlin, swift, react-native, maui, and tauri \
         (multi is handled by the app scaffold command)"
    )]
    UnsupportedFramework { framework: String },
}
