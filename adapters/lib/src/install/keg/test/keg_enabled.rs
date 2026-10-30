#![allow(clippy::unwrap_used)]

/// P-6.3: the transfer layer is opt-in, so no adapter path migrates an
/// existing project's layout unless the user asks for it.
/// (P-6.3: lớp luân chuyển là opt-in, nên không đường adapter nào chuyển layout
/// của project hiện có trừ khi người dùng yêu cầu.)
#[test]
fn keg_layer_is_off_by_default_and_on_only_with_an_explicit_flag() {
    // SAFETY: single-threaded test that restores the variable before returning.
    // AN TOÀN: test đơn luồng, khôi phục biến trước khi trả về.
    #[allow(unsafe_code)]
    unsafe {
        std::env::remove_var("MGC_LIB_KEG");
    }
    assert!(!crate::install::keg::KegStore::is_enabled_by_environment());
    for truthy in ["1", "true"] {
        #[allow(unsafe_code)]
        unsafe {
            std::env::set_var("MGC_LIB_KEG", truthy);
        }
        assert!(
            crate::install::keg::KegStore::is_enabled_by_environment(),
            "{truthy}"
        );
    }
    for falsy in ["0", "false", "yes"] {
        #[allow(unsafe_code)]
        unsafe {
            std::env::set_var("MGC_LIB_KEG", falsy);
        }
        assert!(
            !crate::install::keg::KegStore::is_enabled_by_environment(),
            "{falsy}"
        );
    }
    #[allow(unsafe_code)]
    unsafe {
        std::env::remove_var("MGC_LIB_KEG");
    }
}
