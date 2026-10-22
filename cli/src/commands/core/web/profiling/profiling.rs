// Web command profiling helpers — helpers đo thời gian lệnh Web.
// Web command profiling helpers.

pub(in crate::commands::core::web::implementation) fn web_command_profile_enabled() -> bool {
    std::env::var_os("MAGICORE_WEB_PROFILE_COMMAND").is_some()
}
pub(in crate::commands::core::web::implementation) fn web_command_profile_mark(
    label: &str,
    started_at: std::time::Instant,
) {
    if web_command_profile_enabled() {
        eprintln!(
            "[magicore:web:web-command-profile] {}={}ms",
            label,
            started_at.elapsed().as_millis()
        );
    }
}
