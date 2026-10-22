use std::fs;
use tempfile;

use super::super::*;

#[cfg(unix)]
#[test]
fn linked_cache_or_parent_does_not_modify_the_link_target() {
    use std::os::unix::fs::symlink;
    for linked_parent in [false, true] {
        let root = tempfile::tempdir().unwrap();
        let root = root.path().canonicalize().unwrap();
        let outside = root.join("outside");
        fs::create_dir(&outside).unwrap();
        fs::write(outside.join("sentinel"), b"preserve").unwrap();
        let link = root.join("link");
        symlink(&outside, &link).unwrap();
        let target = if linked_parent {
            link.join("layer")
        } else {
            link
        };

        assert!(extract_tarball_into(&valid_archive(), &target, None).is_err());
        assert_eq!(fs::read(outside.join("sentinel")).unwrap(), b"preserve");
        assert_eq!(fs::read_dir(&outside).unwrap().count(), 1);
    }
}
