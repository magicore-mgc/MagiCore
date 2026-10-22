//! Optional feature catalog data wiring — nối dữ liệu catalog feature tùy chọn.
//! Four contiguous data slices preserve the baseline order and duplicates.

#[path = "application_extensions.rs"]
mod application_extensions;
#[path = "application_foundation.rs"]
mod application_foundation;
#[path = "platform_integrations.rs"]
mod platform_integrations;
#[path = "server_data.rs"]
mod server_data;

pub(super) use application_extensions::APPLICATION_EXTENSIONS;
pub(super) use application_foundation::APPLICATION_FOUNDATION;
pub(super) use platform_integrations::PLATFORM_INTEGRATIONS;
pub(super) use server_data::SERVER_DATA;
