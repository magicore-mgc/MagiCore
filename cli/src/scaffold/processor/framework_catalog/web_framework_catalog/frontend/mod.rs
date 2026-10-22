//! Frontend framework catalog wiring.
//! Wiring catalog framework frontend.

#[path = "frontend.rs"]
mod implementation;

pub(super) use implementation::{
    ANGULAR, ASTRO, NEXTJS, NUXT, QWIK, REACT_VITE, SOLIDJS, SVELTEKIT, VANILLA, VUE_VITE,
};
