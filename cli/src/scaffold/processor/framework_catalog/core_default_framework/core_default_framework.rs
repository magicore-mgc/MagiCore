/// Return the existing default framework for a scaffold core.
/// Trả framework mặc định hiện có cho core scaffold.
pub(in crate::scaffold::processor) fn default_framework(core: &str) -> &'static str {
    match core {
        "web" => "vanilla",
        "game" => "bevy",
        "ai" => "python-agent",
        "clo" => "pulumi-aws",
        "cicd" => "github-actions",
        "iot" => "esp32-rust",
        "app" => "flutter",
        "lib" => "rust",
        _ => "generic",
    }
}
