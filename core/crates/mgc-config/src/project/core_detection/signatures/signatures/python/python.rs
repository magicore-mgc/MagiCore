//! Python project manifest recognition.
//! Nhận diện manifest dự án python.

/// Inspect only dependency sections used by common Python project metadata.
/// Chỉ xét dependency section chuẩn của metadata Python thường dùng.
pub(in crate::project::project::core_detection) fn python_declares_web_framework(
    manifest: &toml::Value,
) -> bool {
    python_declares_named_dependency(
        manifest,
        &[
            "django",
            "fastapi",
            "flask",
            "starlette",
            "sanic",
            "tornado",
            "bottle",
        ],
    )
}

pub(in crate::project::project::core_detection) fn python_declares_ai_framework(
    manifest: &toml::Value,
) -> bool {
    python_declares_named_dependency(
        manifest,
        &[
            "accelerate",
            "anthropic",
            "autogen-agentchat",
            "catboost",
            "cohere",
            "crewai",
            "diffusers",
            "dspy-ai",
            "flax",
            "google-genai",
            "google-generativeai",
            "haystack-ai",
            "jax",
            "keras",
            "langchain",
            "lightgbm",
            "litellm",
            "llama-index",
            "ollama",
            "openai",
            "peft",
            "pydantic-ai",
            "pytorch",
            "scikit-learn",
            "semantic-kernel",
            "sentence-transformers",
            "tensorflow",
            "torch",
            "transformers",
            "vllm",
            "xgboost",
        ],
    )
}

fn python_declares_named_dependency(manifest: &toml::Value, packages: &[&str]) -> bool {
    fn contains_framework(value: &toml::Value, packages: &[&str]) -> bool {
        match value {
            toml::Value::String(spec) => {
                let name = spec
                    .split(['[', ';', '=', '<', '>', '~', '!', ' '])
                    .next()
                    .unwrap_or_default()
                    .trim()
                    .to_ascii_lowercase()
                    .replace('_', "-");
                packages.contains(&name.as_str())
            }
            toml::Value::Array(items) => {
                items.iter().any(|item| contains_framework(item, packages))
            }
            toml::Value::Table(table) => table.iter().any(|(name, value)| {
                let normalized = name.to_ascii_lowercase().replace('_', "-");
                packages.contains(&normalized.as_str()) || contains_framework(value, packages)
            }),
            _ => false,
        }
    }

    let project = manifest.get("project");
    let tool = manifest.get("tool");
    [
        project.and_then(|value| value.get("dependencies")),
        project.and_then(|value| value.get("optional-dependencies")),
        tool.and_then(|value| value.get("poetry"))
            .and_then(|value| value.get("dependencies")),
        tool.and_then(|value| value.get("pdm"))
            .and_then(|value| value.get("dependencies")),
    ]
    .into_iter()
    .flatten()
    .any(|value| contains_framework(value, packages))
}

pub(in crate::project::project::core_detection) fn detect_python_core(
    project_root: &std::path::Path,
) -> anyhow::Result<Option<String>> {
    let path = project_root.join("pyproject.toml");
    if !super::super::shared::signature_file_exists(&path)? {
        return Ok(None);
    }
    let content = crate::project::read_regular_project_text(&path, "Python manifest")?
        .ok_or_else(|| anyhow::anyhow!("Python manifest disappeared during core detection"))?;
    let value: toml::Value = toml::from_str(&content)?;
    Ok(Some(if python_declares_web_framework(&value) {
        "web".to_string()
    } else if python_declares_ai_framework(&value) {
        "ai".to_string()
    } else {
        "lib".to_string()
    }))
}
