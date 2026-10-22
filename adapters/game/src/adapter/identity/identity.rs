use super::super::GameAdapter;
use crate::engine::detect_engine;
use mgc_types::Ecosystem;
use mgc_types::capabilities::CoreIdent;
use std::path::Path;

impl CoreIdent for GameAdapter {
    fn core_id(&self) -> &'static str {
        "game"
    }

    fn name(&self) -> &str {
        "game"
    }

    fn ecosystem(&self) -> Ecosystem {
        Ecosystem::Game
    }
}
impl GameAdapter {
    pub fn detect(root: &Path) -> Option<Self> {
        let engine = detect_engine(root)?;
        Some(Self { engine })
    }

    pub fn engine(&self) -> &'static str {
        self.engine.as_str()
    }
}
