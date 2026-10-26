//! Framework capability decides the engine, not the presence of a script.
//! Năng lực framework quyết định engine, không phải có script hay không.

use super::super::{Engine, Preference, choose};

#[test]
fn a_framework_with_its_own_compiler_keeps_it_even_without_a_script() {
    // Next.js cannot be built here even if the script is missing: guessing
    // MagiCore could reproduce it would produce a broken artefact.
    // Next.js không build được ở đây kể cả khi thiếu script: đoán MagiCore tái
    // tạo được sẽ tạo artefact hỏng.
    for framework in ["next", "nextjs", "nuxt", "angular", "astro", "sveltekit"] {
        let (engine, reason) = choose(Some(framework), false, Preference::Auto);
        assert_eq!(engine, Engine::Framework, "{framework}: {reason}");
    }
}

#[test]
fn a_plain_framework_uses_the_in_process_engine_even_with_a_script() {
    // These are plain TypeScript/JavaScript sources: esbuild parses them
    // directly, so the in-process engine is a real substitute.
    // Đây là source TypeScript/JavaScript thuần: esbuild parse trực tiếp, nên
    // engine trong process là bản thay thế thật.
    for framework in ["react-vite", "vanilla", "solidjs", "preact", "lit"] {
        let (engine, reason) = choose(Some(framework), true, Preference::Auto);
        assert_eq!(engine, Engine::MagiCore, "{framework}: {reason}");
    }
}

#[test]
fn a_single_file_component_framework_keeps_its_compiler() {
    // `.vue` and `.svelte` are not JavaScript: only the framework's own
    // compiler turns them into modules, so esbuild cannot stand in.
    // `.vue` và `.svelte` không phải JavaScript: chỉ compiler của framework biến
    // chúng thành module, nên esbuild không thay được.
    for framework in ["vue-vite", "vue", "sveltekit", "svelte"] {
        let (engine, reason) = choose(Some(framework), true, Preference::Auto);
        assert_eq!(engine, Engine::Framework, "{framework}: {reason}");
    }
}

#[test]
fn forcing_either_engine_is_honoured() {
    assert_eq!(
        choose(Some("next"), true, Preference::MagiCore).0,
        Engine::MagiCore
    );
    assert_eq!(
        choose(Some("react-vite"), true, Preference::Framework).0,
        Engine::Framework
    );
}

#[test]
fn an_unidentified_framework_falls_back_to_its_script() {
    let (engine, reason) = choose(None, true, Preference::Auto);
    assert_eq!(engine, Engine::Framework);
    assert!(reason.contains("no framework identified"), "{reason}");
}
