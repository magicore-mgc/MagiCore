//! Match detected runtimes to adapters, gating rival runtimes behind compat opt-in.
//! Khớp runtime đã phát hiện với adapter, chặn runtime đối thủ nếu chưa opt-in compat.

use super::super::contract::OptimizerAdapter;
use super::super::registry::all_adapters;
use crate::commands::optimizer::runtime_detect::DetectedRuntime;

/// Find adapters matching detected runtimes — tìm adapters khớp với runtimes đã phát hiện
///
/// P0-4 (2026-09-10 native-engine audit): Bun/Deno là runtime ĐỐI THỦ —
/// adapter của chúng chỉ được sinh profile khi dự án CHỌN compat mode
/// tường minh (MGC_COMPAT_RUNTIME / --compat-runtime). Trên đường native
/// mặc định, không sinh profile tối ưu cho runtime đối thủ (tách optimizer
/// rival khỏi optimizer native mặc định).
/// (Bun/Deno adapters only generate profiles under an explicit compat
/// opt-in; the default native lane never generates rival-runtime envs.)
pub fn find_adapters(runtimes: &[DetectedRuntime]) -> Vec<Box<dyn OptimizerAdapter>> {
    let all = all_adapters();
    let mut matched = vec![];

    // Rival runtimes require the explicit compat opt-in (same gate as
    // spawn: cli compat.rs). Without it, Bun/Deno adapters stay dormant.
    // Runtime đối thủ cần compat tường minh (cùng cổng với spawn). Thiếu
    // nó, adapter Bun/Deno ngủ yên.
    let compat = crate::commands::compat::CompatMode::from_flag(None)
        .unwrap_or(crate::commands::compat::CompatMode::Native);
    let rival_allowed = |name: &str| -> bool {
        match compat {
            crate::commands::compat::CompatMode::Native => false,
            crate::commands::compat::CompatMode::Explicit(ref allowed) => {
                allowed.eq_ignore_ascii_case(name)
            }
        }
    };

    for runtime in runtimes {
        for adapter in &all {
            if adapter.matches(runtime) {
                let adapter_name = adapter.name();
                let is_rival = matches!(adapter_name, "Bun" | "Deno");
                if is_rival && !rival_allowed(adapter_name) {
                    continue;
                }
                matched.push(adapter_name);
            }
        }
    }

    // Return fresh instances for matched adapters — trả về instances mới cho adapters khớp
    all.into_iter()
        .filter(|a| matched.contains(&a.name()))
        .collect()
}
