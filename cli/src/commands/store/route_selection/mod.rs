mod route_selection;
#[cfg(test)]
pub(super) use route_selection::{
    LIB_CARGO_ROOTS, LIB_GO_ROOTS, LIB_MAVEN_ROOTS, LIB_NUGET_ROOTS, LIB_PYPI_ROOTS,
    WEB_NATIVE_FRAMEWORKS,
};
pub(super) use route_selection::{StoreRootKind, StoreRoute, store_root_path, store_route};
