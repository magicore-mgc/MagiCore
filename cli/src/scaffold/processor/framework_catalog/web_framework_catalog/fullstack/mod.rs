//! Fullstack catalog wiring by runtime family.
//! Wiring catalog fullstack theo họ runtime.

mod go;
mod java;
mod node;
mod php;
mod python;
mod rust;
mod special;

pub(super) use self::{
    go::{REACT_ECHO, REACT_FIBER, REACT_GIN, SVELTE_GIN, VUE_ECHO, VUE_FIBER, VUE_GIN},
    java::{REACT_QUARKUS, REACT_SPRING, SVELTE_QUARKUS, VUE_QUARKUS},
    node::{
        REACT_EXPRESS, REACT_FASTIFY, REACT_HONO, REACT_NESTJS, REACT_TRPC, SVELTE_EXPRESS,
        SVELTE_HONO, VUE_EXPRESS, VUE_HONO, VUE_NESTJS,
    },
    php::{REACT_SYMFONY, VUE_LARAVEL, VUE_SYMFONY},
    python::{
        REACT_DJANGO, REACT_FASTAPI, REACT_FLASK, SVELTE_FASTAPI, VUE_DJANGO, VUE_FASTAPI,
        VUE_FLASK,
    },
    rust::{REACT_ACTIX_WEB, REACT_AXUM, SVELTE_AXUM, VUE_ACTIX_WEB, VUE_AXUM},
    special::{CUSTOM, REMIX},
};
