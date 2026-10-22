// Validate and render declared template tokens for Web/Core contexts.
// Kiểm tra và render token template đã khai báo cho context Web/Core.
use super::super::super::{CoreTemplateContext, WebTemplateContext};
use super::super::error::TemplateContextError;
use anyhow::Result;
use std::collections::HashSet;

pub(in crate::scaffold::processor) trait ContextValues {
    fn value(&self, key: &str) -> Option<&str>;
}

pub(in crate::scaffold::processor) fn render_contract(
    context: &impl ContextValues,
    input: &str,
    required_context: &[String],
    source: &str,
) -> Result<String> {
    let declared: HashSet<&str> = required_context.iter().map(String::as_str).collect();
    let used = crate::scaffold::processor::extract_template_tokens(input);

    for token in &used {
        if !declared.contains(token.as_str()) {
            return Err(TemplateContextError::UndeclaredToken {
                token: token.clone(),
                template_path: source.to_string(),
            }
            .into());
        }
        if context.value(token).is_none() {
            return Err(TemplateContextError::UnsupportedToken {
                token: token.clone(),
                template_path: source.to_string(),
            }
            .into());
        }
    }

    for key in required_context {
        if context.value(key).is_none() {
            return Err(TemplateContextError::UnsupportedContext {
                key: key.clone(),
                template_path: source.to_string(),
            }
            .into());
        }
    }

    let mut rendered = input.to_string();
    for key in required_context {
        if let Some(value) = context.value(key) {
            rendered = rendered
                .replace(&format!("{{{{{key}}}}}"), value)
                .replace(&format!("{{{{ {key} }}}}"), value);
        }
    }

    Ok(rendered)
}

fn render_project_path_tokens(target: &str, context: &impl ContextValues) -> String {
    let mut rendered = target.to_string();
    for key in ["project_slug", "project_name", "project_package"] {
        if let Some(value) = context.value(key) {
            rendered = rendered
                .replace(&format!("{{{{ {key} }}}}"), value)
                .replace(&format!("{{{{{key}}}}}"), value);
        }
    }
    rendered
}

pub(in crate::scaffold::processor) fn render_target_path(
    target: &str,
    context: &WebTemplateContext,
) -> String {
    render_project_path_tokens(target, context)
}

pub(in crate::scaffold::processor) fn render_core_target_path(
    target: &str,
    context: &CoreTemplateContext,
) -> String {
    render_project_path_tokens(target, context)
}

#[cfg(test)]
#[path = "test/rendering.rs"]
mod tests;
