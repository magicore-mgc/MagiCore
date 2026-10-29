//! mgc telemetry — opt-in status/log (P2, 18 §4)
//! (Zero-telemetry mặc định; user bật bằng env MAGICORE_TELEMETRY=1)

use anyhow::Result;
use clap::Subcommand;

#[derive(Subcommand, Debug, Clone)]
pub enum TelemetryCmd {
    /// Show telemetry status (enabled/disabled)
    Status,
    /// Show locally queued events (transparent — events are never sent anywhere)
    Log,
    /// Enable telemetry (env MAGICORE_TELEMETRY=1, persisted under the MagiCore
    /// data root — run `mgc telemetry status` to see the resolved path)
    /// (Bật telemetry; lưu dưới data root của MagiCore — chạy `mgc telemetry status`
    /// để xem đường dẫn thật)
    On,
    /// Disable telemetry (default state)
    Off,
}

pub fn handle(cmd: TelemetryCmd) -> Result<()> {
    let enabled = mgc_http::telemetry::enabled();
    match cmd {
        TelemetryCmd::Status => {
            println!(
                "telemetry: {} (MAGICORE_TELEMETRY={})",
                if enabled { "ON" } else { "OFF (default)" },
                std::env::var("MAGICORE_TELEMETRY").unwrap_or_default()
            );
            if enabled {
                println!("  queue sends nowhere — writes locally on flush only");
            }
        }
        TelemetryCmd::Log => {
            let path = telemetry_events_path()?;
            if path.exists() {
                println!("{}", path.display());
                let raw = std::fs::read_to_string(&path)?;
                for line in raw.lines() {
                    println!("{line}");
                }
            } else {
                println!("no telemetry events (not enabled or nothing recorded)");
            }
        }
        TelemetryCmd::On | TelemetryCmd::Off => {
            let on = matches!(cmd, TelemetryCmd::On);
            // env persist pick-up: ghi vào profile config để session sau tự nạp
            let cfg_dir = telemetry_config_dir()?;
            std::fs::create_dir_all(&cfg_dir)?;
            let env_file = cfg_dir.join("env");
            let mut lines: Vec<String> = std::fs::read_to_string(&env_file)
                .map(|s| s.lines().map(str::to_string).collect::<Vec<_>>())
                .unwrap_or_default();
            lines.retain(|l| !l.starts_with("MAGICORE_TELEMETRY="));
            lines.push(format!("MAGICORE_TELEMETRY={}", if on { "1" } else { "0" }));
            std::fs::write(&env_file, lines.join("\n") + "\n")?;
            println!(
                "telemetry: {} (wrote {} — the shell prompt must source it; default OFF if absent)",
                if on { "ON" } else { "OFF" },
                cfg_dir.join("env").display()
            );
        }
    }
    Ok(())
}

/// Where the local telemetry event log lives.
///
/// Resolved through `GlobalPaths` rather than `$HOME`: the previous
/// `env::var("HOME").unwrap_or(".")` wrote into the project directory on any
/// system without `HOME` (Windows, minimal containers), dropping a
/// `.config/magicore/env` next to the user's source.
/// (Lấy qua `GlobalPaths` thay vì `$HOME`: cách cũ `unwrap_or(".")` ghi vào thư
/// mục project trên hệ không có `HOME`, tạo `.config/magicore/env` cạnh source
/// của người dùng.)
fn telemetry_events_path() -> Result<std::path::PathBuf> {
    Ok(mgc_platform::paths::GlobalPaths::new()?
        .root
        .join("telemetry/events.jsonl"))
}

/// Where the telemetry enable/disable choice is persisted.
///
/// Kept beside the other MagiCore global state so `mgc telemetry on` and the
/// daemon agree on one directory.
/// (Cạnh state toàn cục khác của MagiCore để `mgc telemetry on` và daemon dùng
/// chung một thư mục.)
fn telemetry_config_dir() -> Result<std::path::PathBuf> {
    Ok(mgc_platform::paths::GlobalPaths::new()?.root.join("config"))
}
