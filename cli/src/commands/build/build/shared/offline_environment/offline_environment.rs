/// Prevent Go's compiler driver from downloading/updating dependency metadata
/// during a MagiCore build. Dependencies must already be present and verified
/// by the selected install path.
pub(in crate::commands::build::implementation) fn go_offline_env() -> Vec<(String, String)> {
    vec![
        ("GOPROXY".to_string(), "off".to_string()),
        ("GOSUMDB".to_string(), "off".to_string()),
        ("GOTOOLCHAIN".to_string(), "local".to_string()),
    ]
}
