//! Builds shared host/port variables for native dev processes.
//! Tạo biến host/port dùng chung cho process dev native.

use std::ffi::OsString;

use super::super::types::NativeDevContext;

pub(in crate::commands::core::web::implementation::dev::native_launch) fn env_host_port_pairs(
    context: &NativeDevContext<'_>,
) -> Vec<(OsString, OsString)> {
    let mut envs = Vec::new();
    if let Some(host) = context.host {
        envs.push((OsString::from("HOST"), OsString::from(host)));
    }
    if let Some(port) = context.port {
        let port = port.to_string();
        envs.push((OsString::from("PORT"), OsString::from(port.clone())));
        envs.push((OsString::from("SERVER_PORT"), OsString::from(port.clone())));
        envs.push((OsString::from("QUARKUS_HTTP_PORT"), OsString::from(port)));
    }
    envs
}
