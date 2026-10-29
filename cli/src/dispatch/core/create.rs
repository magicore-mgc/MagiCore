use anyhow::Result;

use crate::commands;

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

/// Fail closed when the requested framework is not a member of the core.
///
/// An empty framework selects the wizard, which owns its own validation, so it
/// is not gated here.
/// (Fail-closed khi framework không thuộc core. Framework rỗng = wizard tự
/// validate, nên không chặn ở đây.)
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
    Err(crate::error::unknown_framework_for_core(
        core,
        normalized,
        crate::scaffold::framework_registry::supported_frameworks(registry_core),
    ))
}

pub async fn dispatch(command: CoreCommand) -> Result<()> {
    // Security gate first — validate before ANY filesystem access so no
    // garbage directory is created for a rejected name.
    // Chặn traversal/absolute ngay từ CLI boundary — trước mọi truy cập FS.
    if let Some(project_name) = create_project_name(&command) {
        commands::core::create::validate_project_name(&project_name)?;
    }
    if let Some((core, framework)) = create_framework_target(&command) {
        validate_framework_choice(core, framework)?;
    }
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
            project_name,
            source,
        } => commands::core::create::game::run(&framework, &project_name, source).await,
        #[cfg(not(feature = "game"))]
        CoreCommand::CreateGame { .. } => Err(crate::error::core_not_in_build("game")),
        #[cfg(feature = "ai")]
        CoreCommand::CreateAi {
            framework,
            project_name,
            container,
            source,
        } => commands::core::create::ai::run(&framework, &project_name, container, source).await,
        #[cfg(not(feature = "ai"))]
        CoreCommand::CreateAi { .. } => Err(crate::error::core_not_in_build("ai")),
        #[cfg(feature = "clo")]
        CoreCommand::CreateClo {
            framework,
            project_name,
            container,
            source,
        } => commands::core::create::clo::run(&framework, &project_name, container, source).await,
        #[cfg(not(feature = "clo"))]
        CoreCommand::CreateClo { .. } => Err(crate::error::core_not_in_build("clo")),
        #[cfg(feature = "cicd")]
        CoreCommand::CreateCicd {
            framework,
            project_name,
            source,
        } => commands::core::create::cicd::run(&framework, &project_name, source).await,
        #[cfg(not(feature = "cicd"))]
        CoreCommand::CreateCicd { .. } => Err(crate::error::core_not_in_build("cicd")),
        #[cfg(feature = "iot")]
        CoreCommand::CreateIot {
            framework,
            project_name,
            source,
        } => commands::core::create::iot::run(&framework, &project_name, source).await,
        #[cfg(not(feature = "iot"))]
        CoreCommand::CreateIot { .. } => Err(crate::error::core_not_in_build("iot")),
        #[cfg(feature = "app")]
        CoreCommand::CreateApp {
            framework,
            project_name,
            container,
            source,
        } => commands::core::create::app::run(&framework, &project_name, container, source).await,
        #[cfg(not(feature = "app"))]
        CoreCommand::CreateApp { .. } => Err(crate::error::core_not_in_build("app")),
        #[cfg(feature = "lib")]
        CoreCommand::CreateLib {
            framework,
            project_name,
            container,
            source,
        } => {
            commands::core::create::library::run(&framework, &project_name, container, source).await
        }
        #[cfg(not(feature = "lib"))]
        CoreCommand::CreateLib { .. } => Err(crate::error::core_not_in_build("lib")),
        #[cfg(feature = "hardware")]
        CoreCommand::CreateHardware {
            framework,
            project_name,
            source,
        } => commands::core::create::hardware::run(&framework, &project_name, source).await,
        #[cfg(not(feature = "hardware"))]
        CoreCommand::CreateHardware { .. } => Err(crate::error::core_not_in_build("hardware")),
        _ => unreachable!("non-create command routed to create dispatcher"),
    }
}
