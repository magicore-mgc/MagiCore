//! Wire CI/CD adapter construction from detected provider metadata.
//! Nối khởi tạo adapter CICD từ metadata provider được nhận diện.

mod adapter_factory;

pub use adapter_factory::adapter_for;
