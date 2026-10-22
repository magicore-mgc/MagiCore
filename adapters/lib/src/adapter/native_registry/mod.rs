//! Shared native protocol construction for Lib package operations.
//! Dựng protocol native dùng chung cho thao tác package của Lib.

mod dotnet;
mod go;
mod java;
mod metadata;
mod native_registry;
mod python;
mod rust;

pub(in crate::adapter) use metadata::NativeRegistryOperation;
pub(in crate::adapter) use native_registry::NativeRegistry;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
