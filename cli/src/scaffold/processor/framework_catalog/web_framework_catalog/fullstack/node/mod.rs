//! Fullstack node framework owner wiring.
//! Wiring owner framework fullstack node.

#[path = "node.rs"]
mod implementation;

pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) use implementation::{
    REACT_EXPRESS, REACT_FASTIFY, REACT_HONO, REACT_NESTJS, REACT_TRPC, SVELTE_EXPRESS,
    SVELTE_HONO, VUE_EXPRESS, VUE_HONO, VUE_NESTJS,
};
