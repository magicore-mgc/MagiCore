//! Verify tree linking stays safe under nested Rayon execution.
//! Kiểm tra link cây an toàn khi Rayon bị lồng nhau.

use super::super::implementation::hardlink_tree;
use rayon::prelude::*;

#[test]
fn nested_parallel_package_materialization_does_not_reenter_custom_pool() {
    let temp = tempfile::tempdir().expect("tempdir");
    let source = temp.path().join("source");
    std::fs::create_dir_all(source.join("nested")).expect("source dirs");
    for index in 0..64 {
        std::fs::write(
            source.join("nested").join(format!("{index}.js")),
            b"export {}",
        )
        .expect("source file");
    }

    (0..32usize)
        .into_par_iter()
        .try_for_each(|index| hardlink_tree(&source, &temp.path().join(format!("target-{index}"))))
        .expect("nested package materialization");

    assert!(temp.path().join("target-31/nested/63.js").is_file());
}
