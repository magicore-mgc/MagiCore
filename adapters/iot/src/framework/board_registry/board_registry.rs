//! Load and validate the embedded IoT board registry once per process.
//! Nạp và validate registry board IoT nhúng một lần mỗi process.

use crate::framework::{IotFramework, board_types::BoardRecord};
use serde::Deserialize;
use std::sync::OnceLock;

const BOARD_REGISTRY_JSON: &str = include_str!("../../../../../assets/boards/registry.json");

#[derive(Debug, Deserialize)]
struct BoardRegistry {
    schema_version: u32,
    boards: Vec<BoardRecord>,
}

fn board_registry() -> &'static BoardRegistry {
    static REGISTRY: OnceLock<BoardRegistry> = OnceLock::new();
    REGISTRY.get_or_init(|| {
        let registry: BoardRegistry = serde_json::from_str(BOARD_REGISTRY_JSON)
            .expect("embedded IoT board registry must be valid JSON");
        assert_eq!(
            registry.schema_version, 1,
            "unsupported board registry schema"
        );
        assert!(
            !registry.boards.is_empty(),
            "embedded board registry is empty"
        );

        let mut ids = std::collections::HashSet::new();
        let mut choices = std::collections::HashSet::new();
        for board in &registry.boards {
            assert!(!board.id.trim().is_empty(), "board id must not be empty");
            assert!(
                !board.label.trim().is_empty(),
                "board label must not be empty"
            );
            assert!(
                !board.chip.trim().is_empty(),
                "board chip must not be empty"
            );
            assert!(
                !board.target.trim().is_empty(),
                "board target must not be empty"
            );
            assert!(
                ids.insert(board.id.as_str()),
                "duplicate board id in registry"
            );
            assert!(!board.frameworks.is_empty(), "board must name a framework");
            assert!(
                board.frameworks.iter().all(|framework| {
                    IotFramework::from_str(framework)
                        .is_some_and(|parsed| parsed.as_str() == framework)
                }),
                "board registry framework ids must be canonical"
            );
            assert!(
                board
                    .frameworks
                    .iter()
                    .all(|framework| choices.insert((framework.as_str(), board.wizard_order))),
                "board wizard order must be unique within each framework"
            );
        }
        registry
    })
}

pub(in crate::framework) fn board_records() -> &'static [BoardRecord] {
    &board_registry().boards
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
