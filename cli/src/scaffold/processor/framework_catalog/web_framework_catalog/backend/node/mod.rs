//! Backend node framework owner wiring.
//! Wiring owner framework backend node.

#[path = "node.rs"]
mod implementation;

pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) use implementation::{
    EXPRESS, FASTIFY, HONO, NEST, NESTJS, TRPC,
};
