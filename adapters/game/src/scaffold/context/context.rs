//! Template substitution context for game scaffolds.
//! Context thay biến template cho scaffold game.

use crate::engine::GameEngine;
use std::collections::HashMap;

/// Template context cho variable substitution
#[derive(Debug, Clone)]
pub struct TemplateContext {
    pub project_slug: String,
    pub project_name: String,
    pub engine: String,
    pub unity_version: Option<String>,
    pub godot_version: Option<String>,
    pub unreal_version: Option<String>,
}

impl TemplateContext {
    pub fn new(project_name: &str, engine: GameEngine) -> Self {
        let project_slug = project_name
            .to_lowercase()
            .replace(' ', "-")
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-')
            .collect();

        TemplateContext {
            project_slug,
            project_name: project_name.to_string(),
            engine: engine.as_str().to_string(),
            unity_version: None,
            godot_version: None,
            unreal_version: None,
        }
    }

    pub fn to_map(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("project_slug".to_string(), self.project_slug.clone());
        map.insert("project_name".to_string(), self.project_name.clone());
        map.insert("engine".to_string(), self.engine.clone());

        if let Some(ref v) = self.unity_version {
            map.insert("unity_version".to_string(), v.clone());
        }
        if let Some(ref v) = self.godot_version {
            map.insert("godot_version".to_string(), v.clone());
        }
        if let Some(ref v) = self.unreal_version {
            map.insert("unreal_version".to_string(), v.clone());
        }

        map
    }
}
