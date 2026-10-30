//! Guidance must describe remedies that exist and configuration that is read.
//! Hướng dẫn phải nói về cách xử lý có thật và cấu hình được đọc thật.

use super::super::{missing_layer_next_steps, names_nonexistent_subcommand};

#[test]
fn generic_cores_get_the_three_real_remedies_in_a_stable_order() {
    let steps = missing_layer_next_steps("web");
    let local = steps.find("--from-local <path>").expect("local remedy");
    let initializer = steps
        .find("official initializer")
        .expect("initializer remedy");
    let registry = steps.find("MGC_NPM_REGISTRY").expect("registry remedy");
    assert!(local < initializer && initializer < registry, "{steps}");
}

#[test]
fn generator_cores_lead_with_the_offline_generator() {
    let steps = missing_layer_next_steps("game");
    let generator = steps.find("mgc create-game").expect("generator remedy");
    let local = steps.find("--from-local <path>").expect("local remedy");
    assert!(generator < local, "{steps}");
    assert!(steps.contains("scaffolds offline"), "{steps}");
}

#[test]
fn no_core_guidance_names_the_nonexistent_template_subcommand() {
    for core in [
        "web", "ai", "app", "lib", "game", "iot", "clo", "cicd", "hardware",
    ] {
        let steps = missing_layer_next_steps(core);
        assert!(
            !names_nonexistent_subcommand(&steps),
            "core '{core}' names a nonexistent subcommand: {steps}"
        );
    }
}

#[test]
fn the_nonexistent_command_detector_matches_invocations_only() {
    assert!(names_nonexistent_subcommand(
        "run: mgc template fetch web x@latest"
    ));
    assert!(!names_nonexistent_subcommand(
        "see the mgc template module docs"
    ));
    assert!(names_nonexistent_subcommand(
        "then run mgc template fetch <url>"
    ));
}
