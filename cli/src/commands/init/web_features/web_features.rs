//! Web feature defaults and interactive selection.
//! Mặc định và lựa chọn feature Web.

use crate::wizard::engine::{Answer, ScaffoldConfig};

pub(in crate::commands::init) fn ask_web_features(config: &ScaffoldConfig) -> (Vec<Answer>, bool) {
    let use_defaults =
        mgc_ui::prompt::confirm("Use default settings for this framework?").unwrap_or(true);

    let options = match config.sub_type.as_str() {
        "frontend" => vec![
            Answer::new("TypeScript", "typescript"),
            Answer::new("Tailwind CSS", "tailwindcss"),
            Answer::new("ESLint", "eslint"),
            // Answer::new("Vitest", "vitest"),  // DISABLED: vitest crashes (known issue V1.0)
            Answer::new("Jest", "jest"),
            Answer::new("Playwright", "playwright"),
        ],
        "backend" => vec![
            Answer::new("TypeScript", "typescript"),
            Answer::new("OpenAPI contract", "openapi"),
            Answer::new("ESLint", "eslint"),
            Answer::new("Database layer", "db"),
            Answer::new("Container baseline", "docker"),
        ],
        "fullstack" => vec![
            Answer::new("TypeScript", "typescript"),
            Answer::new("Tailwind CSS", "tailwindcss"),
            Answer::new("Shared schema/contracts", "schema"),
            Answer::new("API client generation", "api-client"),
            Answer::new("Playwright", "playwright"),
            Answer::new("Database layer", "db"),
        ],
        "monorepo" => vec![
            Answer::new("TypeScript", "typescript"),
            Answer::new("Shared schema/contracts", "schema"),
            Answer::new("Shared config layer", "shared-config"),
            Answer::new("API client generation", "api-client"),
            Answer::new("Workspace lint baseline", "eslint"),
            Answer::new("Playwright", "playwright"),
        ],
        _ => vec![],
    };

    if use_defaults {
        (options.into_iter().take(2).collect(), false)
    } else {
        (options, true)
    }
}
