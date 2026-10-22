//! `mgc init` workflow coordinator and owner wiring.
//! Điều phối luồng `mgc init` và nối các owner.

use crate::scaffold::Scaffolder;
use crate::wizard::engine::{Question, QuestionKind, ScaffoldConfig, WizardEngine};
use crate::wizard::web::WebWizard;
use anyhow::Result;
use mgc_config::project::ProjectConfig;
use mgc_ui::{print_banner, print_next_steps, section};

#[path = "core_wizard/mod.rs"]
mod core_wizard;
pub(super) use core_wizard::{iot_wizard_config, pick_core, run_core_wizard};
#[path = "hardware_project/mod.rs"]
mod hardware_project;
pub(super) use hardware_project::init_hardware;
#[path = "project_metadata/mod.rs"]
mod project_metadata;
pub(super) use project_metadata::write_mgc_toml;
#[path = "project_name/mod.rs"]
mod project_name;
pub(super) use project_name::ask_project_name;
#[path = "web_dependency_seed/mod.rs"]
mod web_dependency_seed;
pub(super) use web_dependency_seed::seed_web_deps;
#[path = "web_features/mod.rs"]
mod web_features;
pub(super) use web_features::ask_web_features;

/// `mgc init` — create a new project with `mgc.toml`.
/// `mgc init` — tạo dự án mới cùng `mgc.toml`.
///
/// `--signature <core>` remains a compatibility flag that writes a plain-text
/// `.mgc.core` identity marker; it does not create a cryptographic signature.
/// (Flag tương thích này ghi marker core dạng text, không tạo chữ ký mật mã.)
pub async fn run(template: Option<String>, signature: Option<String>) -> Result<()> {
    if let Some(core) = signature {
        let cwd = std::env::current_dir().map_err(|e| crate::error::cwd_deleted(&e))?;
        let root = ProjectConfig::find_project_root(&cwd).unwrap_or(cwd);
        ProjectConfig::write_core_marker_at(&root, &core)?;
        mgc_ui::success(&format!(
            "Core identity marker '{}' written to {}/{} (plain text; not cryptographically signed)",
            core,
            root.display(),
            ProjectConfig::CORE_MARKER_FILE,
        ));
        return Ok(());
    }
    print_banner();

    if let Some(t) = template {
        let mut config = if t == "web" {
            WebWizard::run()
        } else if t == "hardware" {
            crate::wizard::hardware::HardwareWizard::run()
        } else if t == "lib" {
            crate::wizard::lib::LibWizard::run()
        } else if t == "game" {
            crate::wizard::game::GameWizard::run()
        } else if t == "iot" {
            iot_wizard_config()?
        } else if t == "clo" || t == "cloud" {
            crate::wizard::cloud::CloudWizard::run()
        } else if t == "cicd" {
            crate::wizard::cicd::CicdWizard::run()
        } else if t == "app" {
            crate::wizard::app::AppWizard::run()
        } else if t == "ai" {
            crate::wizard::ai::AiWizard::run()
        } else {
            ScaffoldConfig {
                core: t.clone(),
                sub_type: String::new(),
                frameworks: vec![],
                project_name: String::new(),
                features: vec![],
                template_dir: std::path::PathBuf::new(),
            }
        };
        config.project_name = ask_project_name()?;
        if t == "web" {
            let (features, show_multi) = ask_web_features(&config);
            if !features.is_empty() {
                if show_multi {
                    let feats = WizardEngine::run_question(&Question {
                        prompt: "Select features:".to_string(),
                        kind: QuestionKind::MultiSelect { options: features },
                    });
                    config.features = feats;
                } else {
                    config.features = features.into_iter().map(|a| a.value).collect();
                }
            }
        }
        if t == "hardware" {
            let project_dir = init_hardware(&config).await?;
            write_mgc_toml(&project_dir, &config)?;
            return Ok(());
        }
        let project_dir = Scaffolder::scaffold(&config)?;
        write_mgc_toml(&project_dir, &config)?;
        if t == "web" && !config.frameworks.is_empty() {
            seed_web_deps(&project_dir, &config).await?;
        }
        return Ok(());
    }

    section("Choose your project type", 1, 4);
    let core = pick_core();

    section("Configure your project", 2, 4);

    let (mut config, features, show_multi) = run_core_wizard(&core)?;

    section("Additional features", 3, 4);
    if !features.is_empty() {
        if show_multi {
            let feats = WizardEngine::run_question(&Question {
                prompt: "Select features:".to_string(),
                kind: QuestionKind::MultiSelect { options: features },
            });
            config.features = feats;
        } else {
            config.features = features.into_iter().map(|a| a.value).collect();
        }
    }

    section("Creating project...", 4, 4);
    let project_dir = if config.core == "hardware" {
        init_hardware(&config).await?
    } else {
        Scaffolder::scaffold(&config)?
    };

    write_mgc_toml(&project_dir, &config)?;

    if config.core == "web" && !config.frameworks.is_empty() {
        seed_web_deps(&project_dir, &config).await?;
    }

    print_next_steps(&config.project_name);
    Ok(())
}

#[cfg(test)]
#[path = "test/init.rs"]
mod tests;
