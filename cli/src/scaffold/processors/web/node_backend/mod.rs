//! Node/TypeScript Web backend starter owner.
//! Owner starter backend Web Node/TypeScript.

mod node_backend;

pub(crate) use node_backend::{write_fastify, write_hono, write_trpc};
