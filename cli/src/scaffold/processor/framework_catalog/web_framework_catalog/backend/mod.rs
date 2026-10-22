//! Backend catalog wiring by runtime family.
//! Wiring catalog backend theo họ runtime.

mod dotnet;
mod go;
mod java;
mod node;
mod php;
mod python;
mod rust;

pub(super) use self::{
    dotnet::{DOTNET_MINIMAL, DOTNET_WEBAPI},
    go::{ECHO, FIBER, GIN},
    java::{QUARKUS, SPRING_BOOT},
    node::{EXPRESS, FASTIFY, HONO, NESTJS, TRPC},
    php::{LARAVEL, SYMFONY},
    python::{DJANGO, FASTAPI, FLASK},
    rust::{ACTIX_WEB, AXUM},
};
