//! Build-feature visibility contract for the shared factory.
//! Contract core khả dụng theo feature của factory dùng chung.

#[test]
fn test_available_cores_matches_build_shape() {
    let available = mgc::factory::available_cores();

    #[cfg(feature = "web")]
    assert!(available.iter().any(|(core, _)| *core == "web"));
    #[cfg(feature = "ai")]
    assert!(available.iter().any(|(core, _)| *core == "ai"));
    #[cfg(feature = "app")]
    assert!(available.iter().any(|(core, _)| *core == "app"));
    #[cfg(feature = "lib")]
    assert!(available.iter().any(|(core, _)| *core == "lib"));
    #[cfg(feature = "game")]
    assert!(available.iter().any(|(core, _)| *core == "game"));
    #[cfg(feature = "iot")]
    assert!(available.iter().any(|(core, _)| *core == "iot"));
    #[cfg(feature = "clo")]
    assert!(available.iter().any(|(core, _)| *core == "clo"));
    #[cfg(feature = "cicd")]
    assert!(available.iter().any(|(core, _)| *core == "cicd"));
    #[cfg(feature = "hardware")]
    assert!(available.iter().any(|(core, _)| *core == "hardware"));
}
