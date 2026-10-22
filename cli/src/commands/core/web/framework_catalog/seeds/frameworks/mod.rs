// Framework-specific seed owners — owner seed riêng cho từng framework.
// This file wires the leaf data modules and exposes their constants to the ordered catalog.
use super::types::{WebFrameworkSeed, WebToolchainPackage};

mod angular;
mod astro;
mod express;
mod fastify;
mod hono;
mod nestjs;
mod nextjs;
mod nuxt;
mod qwik;
mod react_vite;
mod solidjs;
mod sveltekit;
mod trpc;
mod vanilla;
mod vue_vite;

pub(super) use angular::SEED as ANGULAR_SEED;
pub(super) use astro::SEED as ASTRO_SEED;
pub(super) use express::SEED as EXPRESS_SEED;
pub(super) use fastify::SEED as FASTIFY_SEED;
pub(super) use hono::SEED as HONO_SEED;
pub(super) use nestjs::SEED as NESTJS_SEED;
pub(super) use nextjs::SEED as NEXTJS_SEED;
pub(super) use nuxt::SEED as NUXT_SEED;
pub(super) use qwik::SEED as QWIK_SEED;
pub(super) use react_vite::SEED as REACT_VITE_SEED;
pub(super) use solidjs::SEED as SOLIDJS_SEED;
pub(super) use sveltekit::SEED as SVELTEKIT_SEED;
pub(super) use trpc::SEED as TRPC_SEED;
pub(super) use vanilla::SEED as VANILLA_SEED;
pub(super) use vue_vite::SEED as VUE_VITE_SEED;
