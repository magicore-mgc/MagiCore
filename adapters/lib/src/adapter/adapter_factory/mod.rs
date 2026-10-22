//! Public LibAdapter constructor functions.
//! Hàm dựng LibAdapter công khai.

mod adapter_factory;
pub use adapter_factory::{
    adapter_for, adapter_for_ai_python, adapter_for_language, adapter_for_web_backend,
    adapter_for_with_chain, adapter_for_with_scoped_chain,
};
