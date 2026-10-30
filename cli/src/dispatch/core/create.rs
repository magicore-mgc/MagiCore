use anyhow::Result;

use crate::commands;
use crate::commands::core::create::automation::{CreateAutomationFlags, resolve_automation};

use super::super::types::CoreCommand;

pub fn matches(command: &CoreCommand) -> bool {
    matches!(
        command,
        CoreCommand::CreateWeb { .. }
            | CoreCommand::CreateGame { .. }
            | CoreCommand::CreateAi { .. }
            | CoreCommand::CreateClo { .. }
            | CoreCommand::CreateCicd { .. }
            | CoreCommand::CreateIot { .. }
            | CoreCommand::CreateApp { .. }
            | CoreCommand::CreateLib { .. }
            | CoreCommand::CreateHardware { .. }
    )
}

/// Extract the user-supplied project name from any create-* command.
/// Lấy project name từ mọi create-* command để validate đồng nhất.
fn create_project_name(command: &CoreCommand) -> Option<String> {
    match command {
        CoreCommand::CreateWeb { project_name, .. }
        | CoreCommand::CreateGame { project_name, .. }
        | CoreCommand::CreateAi { project_name, .. }
        | CoreCommand::CreateClo { project_name, .. }
        | CoreCommand::CreateCicd { project_name, .. }
        | CoreCommand::CreateIot { project_name, .. }
        | CoreCommand::CreateApp { project_name, .. }
        | CoreCommand::CreateLib { project_name, .. }
        | CoreCommand::CreateHardware { project_name, .. } => Some(project_name.clone()),
        _ => None,
    }
}

/// Borrow the automation flags from any automation-parity create-* command.
///
/// Returns the core label, framework, positional project name and the flags so
/// the dispatcher can resolve `--name`/`--yes` before touching the filesystem.
/// (Mượn cờ tự động hoá từ mọi create-* có cờ tự động hoá. Trả về core,
/// framework, tên project vị trí và cờ để dispatcher phân giải `--name`/
/// `--yes` trước khi chạm filesystem.)
fn create_automation(
    command: &CoreCommand,
) -> Option<(&'static str, &str, &str, &CreateAutomationFlags)> {
    let (core, framework, project_name, automation) = match command {
        CoreCommand::CreateGame {
            framework,
            project_name,
            automation,
            ..
        } => (
            "game",
            framework.as_str(),
            project_name.as_str(),
            automation,
        ),
        CoreCommand::CreateAi {
            framework,
            project_name,
            automation,
            ..
        } => ("ai", framework.as_str(), project_name.as_str(), automation),
        CoreCommand::CreateClo {
            framework,
            project_name,
            automation,
            ..
        } => ("clo", framework.as_str(), project_name.as_str(), automation),
        CoreCommand::CreateCicd {
            framework,
            project_name,
            automation,
            ..
        } => (
            "cicd",
            framework.as_str(),
            project_name.as_str(),
            automation,
        ),
        CoreCommand::CreateIot {
            framework,
            project_name,
            automation,
            ..
        } => ("iot", framework.as_str(), project_name.as_str(), automation),
        CoreCommand::CreateApp {
            framework,
            project_name,
            automation,
            ..
        } => ("app", framework.as_str(), project_name.as_str(), automation),
        CoreCommand::CreateLib {
            framework,
            project_name,
            automation,
            ..
        } => ("lib", framework.as_str(), project_name.as_str(), automation),
        CoreCommand::CreateHardware {
            framework,
            project_name,
            automation,
            ..
        } => (
            "hardware",
            framework.as_str(),
            project_name.as_str(),
            automation,
        ),
        _ => return None,
    };
    Some((core, framework, project_name, automation))
}

/// Extract the requested framework and the core it belongs to.
///
/// The registry gate runs on this pair before any scaffold work, so a typo or
/// a name from another core fails with the list of names this core accepts
/// instead of writing `mgc.toml` for a project nothing can build.
/// (Lấy framework + core tương ứng. Cổng registry chạy trên cặp này TRƯỚC mọi
/// bước scaffold, nên gõ sai hoặc dùng tên của core khác sẽ fail kèm danh sách
/// tên core này nhận, thay vì ghi `mgc.toml` cho project không build được.)
fn create_framework_target(command: &CoreCommand) -> Option<(&'static str, &str)> {
    let (core, framework) = match command {
        CoreCommand::CreateWeb { framework, .. } => ("web", framework.as_str()),
        CoreCommand::CreateGame { framework, .. } => ("game", framework.as_str()),
        CoreCommand::CreateAi { framework, .. } => ("ai", framework.as_str()),
        CoreCommand::CreateClo { framework, .. } => ("clo", framework.as_str()),
        CoreCommand::CreateCicd { framework, .. } => ("cicd", framework.as_str()),
        CoreCommand::CreateIot { framework, .. } => ("iot", framework.as_str()),
        CoreCommand::CreateApp { framework, .. } => ("app", framework.as_str()),
        CoreCommand::CreateLib { framework, .. } => ("lib", framework.as_str()),
        CoreCommand::CreateHardware { framework, .. } => ("hardware", framework.as_str()),
        _ => return None,
    };
    Some((core, framework))
}

/// Fail closed when the requested framework is not scaffoldable for the core.
///
/// Two honest failures are possible and they must not be merged:
///
/// - a Web backend MagiCore knows but cannot scaffold yet (no initializer
///   record) reports "not yet supported for scaffolding" — the name is real,
///   the roadmap is pending;
/// - anything else reports the unknown-framework error with the accepted list.
///
/// An empty framework selects the wizard, which owns its own validation, so it
/// is not gated here.
/// (Fail-closed khi framework không scaffold được cho core. Hai lỗi trung thực
/// không được gộp: backend Web đã biết nhưng chưa scaffold được báo "not yet
/// supported for scaffolding"; mọi trường hợp khác báo unknown-framework kèm
/// danh sách. Framework rỗng = wizard tự validate, không chặn ở đây.)
fn validate_framework_choice(core: &'static str, framework: &str) -> Result<()> {
    let normalized = framework.trim();
    if normalized.is_empty() {
        return Ok(());
    }
    let with_version = normalized
        .split_once('@')
        .map(|(name, _)| name)
        .unwrap_or(normalized);
    let registry_core = crate::scaffold::framework_registry::ecosystem_for_core(core);
    if crate::scaffold::framework_registry::is_supported(registry_core, with_version) {
        return Ok(());
    }
    let supported = crate::scaffold::framework_registry::supported_frameworks(registry_core);
    if crate::scaffold::framework_registry::is_known_web_unsupported(with_version) {
        return Err(crate::error::web_framework_not_yet_scaffoldable(
            normalized, supported,
        ));
    }
    Err(crate::error::unknown_framework_for_core(
        core, normalized, supported,
    ))
}

pub async fn dispatch(command: CoreCommand) -> Result<()> {
    // Resolve automation flags first: `--name` replaces the positional PROJECT
    // argument and `--yes` fails closed when a prompt would otherwise be needed.
    // Phân giải cờ tự động hoá trước: `--name` thay tham số PROJECT vị trí và
    // `--yes` fail-closed khi nếu không sẽ phải hỏi.
    let resolved = match create_automation(&command) {
        Some((core, framework, positional, automation)) => {
            Some(resolve_automation(core, framework, positional, automation)?)
        }
        None => None,
    };
    // The resolved `--name` wins over the positional value for validation and
    // for the lane; `--yes` never reaches an interactive prompt.
    // Tên đã phân giải (`--name`) thắng giá trị vị trí khi validate và khi gọi
    // lane; `--yes` không bao giờ chạm prompt tương tác.
    let effective_name = match &resolved {
        Some(resolved) => resolved.project_name.clone(),
        None => create_project_name(&command).unwrap_or_default(),
    };
    // Security gate first — validate before ANY filesystem access so no
    // garbage directory is created for a rejected name.
    // Chặn traversal/absolute ngay từ CLI boundary — trước mọi truy cập FS.
    if !effective_name.is_empty() {
        commands::core::create::validate_project_name(&effective_name)?;
    }
    if let Some((core, framework)) = create_framework_target(&command) {
        validate_framework_choice(core, framework)?;
    }
    let lane_name = effective_name.as_str();
    match command {
        #[cfg(feature = "web")]
        CoreCommand::CreateWeb {
            framework,
            project_name,
            flags,
        } => {
            commands::core::create::web::run_create_with_options(
                &framework,
                &project_name,
                Some(flags),
            )
            .await
        }
        #[cfg(not(feature = "web"))]
        CoreCommand::CreateWeb { .. } => Err(crate::error::core_not_in_build("web")),
        #[cfg(feature = "game")]
        CoreCommand::CreateGame {
            framework,
            project_name: _,
            source,
            automation: _,
        } => commands::core::create::game::run(&framework, lane_name, source).await,
        #[cfg(not(feature = "game"))]
        CoreCommand::CreateGame { .. } => Err(crate::error::core_not_in_build("game")),
        #[cfg(feature = "ai")]
        CoreCommand::CreateAi {
            framework,
            project_name: _,
            container,
            source,
            automation: _,
        } => commands::core::create::ai::run(&framework, lane_name, container, source).await,
        #[cfg(not(feature = "ai"))]
        CoreCommand::CreateAi { .. } => Err(crate::error::core_not_in_build("ai")),
        #[cfg(feature = "clo")]
        CoreCommand::CreateClo {
            framework,
            project_name: _,
            container,
            source,
            automation: _,
        } => commands::core::create::clo::run(&framework, lane_name, container, source).await,
        #[cfg(not(feature = "clo"))]
        CoreCommand::CreateClo { .. } => Err(crate::error::core_not_in_build("clo")),
        #[cfg(feature = "cicd")]
        CoreCommand::CreateCicd {
            framework,
            project_name: _,
            source,
            automation: _,
        } => commands::core::create::cicd::run(&framework, lane_name, source).await,
        #[cfg(not(feature = "cicd"))]
        CoreCommand::CreateCicd { .. } => Err(crate::error::core_not_in_build("cicd")),
        #[cfg(feature = "iot")]
        CoreCommand::CreateIot {
            framework,
            project_name: _,
            source,
            automation: _,
        } => commands::core::create::iot::run(&framework, lane_name, source).await,
        #[cfg(not(feature = "iot"))]
        CoreCommand::CreateIot { .. } => Err(crate::error::core_not_in_build("iot")),
        #[cfg(feature = "app")]
        CoreCommand::CreateApp {
            framework,
            project_name: _,
            container,
            source,
            automation: _,
        } => commands::core::create::app::run(&framework, lane_name, container, source).await,
        #[cfg(not(feature = "app"))]
        CoreCommand::CreateApp { .. } => Err(crate::error::core_not_in_build("app")),
        #[cfg(feature = "lib")]
        CoreCommand::CreateLib {
            framework,
            project_name: _,
            container,
            source,
            automation: _,
        } => commands::core::create::library::run(&framework, lane_name, container, source).await,
        #[cfg(not(feature = "lib"))]
        CoreCommand::CreateLib { .. } => Err(crate::error::core_not_in_build("lib")),
        #[cfg(feature = "hardware")]
        CoreCommand::CreateHardware {
            framework,
            project_name: _,
            source,
            automation: _,
        } => commands::core::create::hardware::run(&framework, lane_name, source).await,
        #[cfg(not(feature = "hardware"))]
        CoreCommand::CreateHardware { .. } => Err(crate::error::core_not_in_build("hardware")),
        _ => unreachable!("non-create command routed to create dispatcher"),
    }
}
