use super::super::GameAdapter;
use crate::engine::detect_engine;
use std::path::Path;

pub fn adapter_for(root: &Path) -> Option<GameAdapter> {
    let engine = detect_engine(root)?;
    Some(GameAdapter { engine })
}
