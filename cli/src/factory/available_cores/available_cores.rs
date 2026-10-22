/// Available cores in this build (for init menu filtering)
#[allow(clippy::vec_init_then_push)]
pub fn available_cores() -> Vec<(&'static str, &'static str)> {
    let mut cores = Vec::new();
    #[cfg(feature = "web")]
    cores.push(("web", "🌐  Web application"));
    #[cfg(feature = "lib")]
    cores.push(("lib", "📚  Library (ts / rust / python)"));
    #[cfg(feature = "ai")]
    cores.push(("ai", "🤖  AI agent / model runtime"));
    #[cfg(feature = "app")]
    cores.push(("app", "📱  App (flutter / kotlin / swift)"));
    #[cfg(feature = "game")]
    cores.push(("game", "🎮  Game (bevy / godot / unity / unreal)"));
    #[cfg(feature = "iot")]
    cores.push(("iot", "📡  IoT (esp32-rust / platformio / zephyr)"));
    #[cfg(feature = "clo")]
    cores.push(("clo", "☁️  Cloud infrastructure"));
    #[cfg(feature = "cicd")]
    cores.push(("cicd", "🚦  CI/CD pipeline"));
    #[cfg(feature = "hardware")]
    cores.push((
        "hardware",
        "⚙️  Hardware (optimizer/bench — CPU/RAM tuning)",
    ));
    cores
}
