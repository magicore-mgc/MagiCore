//! Tests profile quoting, idempotence and malformed ownership markers.
//! Kiểm tra quote profile, idempotence và marker ownership sai.

use super::super::{managed_path_profile_block, replace_managed_profile_block};
use std::path::Path;

#[test]
fn rendered_zsh_block_quotes_spaces_and_is_idempotent() {
    let bin = Path::new("/tmp/MagiCore data/bin");
    let block = managed_path_profile_block(bin, false).unwrap();
    assert!(block.contains("'/tmp/MagiCore data/bin'"));
    assert!(block.contains("case \":$PATH:\""));
    let once = replace_managed_profile_block("# user config\n", &block).unwrap();
    let twice = replace_managed_profile_block(&once, &block).unwrap();
    assert_eq!(once, twice);
}

#[test]
fn malformed_or_duplicate_markers_fail_closed() {
    let desired = managed_path_profile_block(Path::new("/opt/mgc/bin"), false).unwrap();
    for contents in [
        "# >>> magicore-managed-tool-bin >>>\nmissing end\n",
        "# >>> magicore-managed-tool-bin >>>\n# <<< magicore-managed-tool-bin <<<\n# >>> magicore-managed-tool-bin >>>\n# <<< magicore-managed-tool-bin <<<\n",
    ] {
        assert!(replace_managed_profile_block(contents, &desired).is_err());
    }
}
