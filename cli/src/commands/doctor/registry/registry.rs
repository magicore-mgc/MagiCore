//! Maps configured outbound connections to doctor registry statuses.
//! Ánh xạ outbound connection cấu hình thành registry status của doctor.

use super::super::model::RegistryStatus;

pub(in crate::commands::doctor) fn statuses() -> Vec<RegistryStatus> {
    let connections = crate::commands::network::outbound_connections();
    build_statuses(
        connections
            .into_iter()
            .map(|connection| (connection.host, connection.port)),
        crate::commands::network::reachable_public,
    )
}

pub(in crate::commands::doctor) fn build_statuses<I, F>(
    connections: I,
    mut reachable: F,
) -> Vec<RegistryStatus>
where
    I: IntoIterator<Item = (String, u16)>,
    F: FnMut(&str, u16) -> bool,
{
    connections
        .into_iter()
        .map(|(host, port)| RegistryStatus {
            reachable: reachable(&host, port),
            host,
        })
        .collect()
}
