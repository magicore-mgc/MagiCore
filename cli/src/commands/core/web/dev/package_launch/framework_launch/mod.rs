//! Wires independent frontend framework launchers in existing dispatch order.
//! Nối launcher frontend độc lập theo thứ tự dispatch hiện tại.

mod angular;
mod astro;
mod framework_launch;
mod next;
mod nuxt;
mod remix;
mod vite;

pub(in crate::commands::core::web::implementation::dev::package_launch) use framework_launch::resolve_framework_launch;
