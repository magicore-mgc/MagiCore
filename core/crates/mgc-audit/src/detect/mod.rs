//! Shared language-manifest detection exports.
//! Export các rule nhận diện manifest dùng chung theo ngôn ngữ.

mod dotnet;
mod go;
mod java;
mod python;
mod rust;

pub use dotnet::dotnet_step;
pub use go::go_step;
pub use java::java_step;
pub use python::python_step;
pub use rust::rust_step;
