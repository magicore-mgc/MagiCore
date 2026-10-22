#![allow(clippy::unwrap_used)]
// virtual-store path tests — kiểm thử đường dẫn virtual-store.
// Scoped package paths must keep their project and scope boundaries.
// Đường dẫn package scoped phải giữ ranh giới project và scope.
use super::*;
use crate::install::materialize::test::support::package_id;

#[test]
fn scoped_package_paths_keep_scope_boundary_in_project_node_modules() {
    let modules = Path::new("/project/node_modules");
    let package = package_id("@scope/pkg@1.2.3");

    assert_eq!(
        strict_vstore_node_modules_dir(modules, &package),
        PathBuf::from("/project/node_modules/.magicore/@scope+pkg@1.2.3/node_modules"),
    );
    assert_eq!(
        strict_vstore_package_dir(modules, &package),
        PathBuf::from("/project/node_modules/.magicore/@scope+pkg@1.2.3/node_modules/@scope/pkg"),
    );
}
