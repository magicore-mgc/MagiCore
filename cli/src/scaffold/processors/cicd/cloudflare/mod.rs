//! Cloudflare Worker output owner wiring.
//! Wiring owner output Cloudflare Worker.

#[path = "cloudflare.rs"]
mod implementation;
pub(super) use implementation::write;
