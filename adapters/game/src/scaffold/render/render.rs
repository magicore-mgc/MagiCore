//! Placeholder substitution for game templates.
//! Thay placeholder cho template game.

use std::collections::HashMap;

/// Render template string với context variables
pub fn render_template(template: &str, context: &HashMap<String, String>) -> String {
    let mut result = template.to_string();

    for (key, value) in context {
        let placeholder = format!("{{{{{}}}}}", key);
        result = result.replace(&placeholder, value);
    }

    result
}
