//! Generator-local typed error boundary — Ranh giới lỗi typed cục bộ của generator

mod error;

pub(crate) use self::error::{GeneratorError, GeneratorResult};
