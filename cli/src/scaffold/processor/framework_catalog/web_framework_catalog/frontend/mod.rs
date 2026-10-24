//! Frontend framework catalog wiring.
//! Wiring catalog framework frontend.

#[path = "frontend.rs"]
mod implementation;

pub(super) use implementation::{
    ANGULAR, ASTRO, DOCUSAURUS_MGC, LIT_MGC, NEXTJS, NUXT, PREACT_MGC, QWIK, REACT_VITE, SOLIDJS,
    SVELTEKIT, TAURI_MGC, TURBO_MGC, VANILLA, VITE_MGC, VITEPRESS_MGC, VUE_CLI_MGC, VUE_VITE,
};
