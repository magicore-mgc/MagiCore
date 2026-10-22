//! Core selection and wizard routing for init.
//! Chọn core và định tuyến wizard trong init.

use super::{ask_project_name, ask_web_features};
use crate::factory;
use crate::wizard::engine::{Answer, Question, QuestionKind, ScaffoldConfig, WizardEngine};
use crate::wizard::web::WebWizard;
use anyhow::Result;

pub(in crate::commands::init) fn iot_wizard_config() -> Result<ScaffoldConfig> {
    #[cfg(feature = "iot")]
    {
        Ok(crate::wizard::iot::IotWizard::run())
    }
    #[cfg(not(feature = "iot"))]
    {
        Err(crate::error::core_not_in_build("iot"))
    }
}

pub(in crate::commands::init) fn pick_core() -> String {
    let avail = factory::available_cores();
    if avail.is_empty() {
        eprintln!("error: no cores available in this build");
        std::process::exit(1);
    }
    let options: Vec<Answer> = avail
        .iter()
        .map(|(short, label)| Answer::new(label, short))
        .collect();

    let question = Question {
        prompt: "What do you want to build?".to_string(),
        kind: QuestionKind::Select { options },
    };
    WizardEngine::run_question(&question)
        .first()
        .cloned()
        .unwrap_or_else(|| avail[0].0.to_string())
}

pub(in crate::commands::init) fn run_core_wizard(
    core: &str,
) -> Result<(ScaffoldConfig, Vec<Answer>, bool)> {
    match core {
        "web" => {
            let mut cfg = WebWizard::run();
            cfg.project_name = ask_project_name()?;
            let (features, show_multi) = ask_web_features(&cfg);
            Ok((cfg, features, show_multi))
        }
        "hardware" => {
            let mut cfg = crate::wizard::hardware::HardwareWizard::run();
            cfg.project_name = ask_project_name()?;
            Ok((cfg, Vec::new(), false))
        }
        "lib" => {
            let mut cfg = crate::wizard::lib::LibWizard::run();
            cfg.project_name = ask_project_name()?;
            Ok((cfg, Vec::new(), false))
        }
        "game" => {
            let mut cfg = crate::wizard::game::GameWizard::run();
            cfg.project_name = ask_project_name()?;
            Ok((cfg, Vec::new(), false))
        }
        "iot" => {
            let mut cfg = iot_wizard_config()?;
            cfg.project_name = ask_project_name()?;
            Ok((cfg, Vec::new(), false))
        }
        "clo" | "cloud" => {
            let mut cfg = crate::wizard::cloud::CloudWizard::run();
            cfg.project_name = ask_project_name()?;
            Ok((cfg, Vec::new(), false))
        }
        "cicd" => {
            let mut cfg = crate::wizard::cicd::CicdWizard::run();
            cfg.project_name = ask_project_name()?;
            Ok((cfg, Vec::new(), false))
        }
        "app" => {
            let mut cfg = crate::wizard::app::AppWizard::run();
            cfg.project_name = ask_project_name()?;
            Ok((cfg, Vec::new(), false))
        }
        "ai" => {
            let mut cfg = crate::wizard::ai::AiWizard::run();
            cfg.project_name = ask_project_name()?;
            Ok((cfg, Vec::new(), false))
        }
        _ => {
            let name = ask_project_name()?;
            Ok((
                ScaffoldConfig {
                    core: core.to_string(),
                    project_name: name,
                    ..Default::default()
                },
                Vec::new(),
                false,
            ))
        }
    }
}
