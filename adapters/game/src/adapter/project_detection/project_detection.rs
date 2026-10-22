use super::super::GameAdapter;
use crate::engine::manifest_is_game;
use mgc_types::capabilities::ProjectDetector;
use std::path::Path;

impl ProjectDetector for GameAdapter {
    fn can_handle(&self, project_root: &Path) -> bool {
        manifest_is_game(project_root)
    }
}
