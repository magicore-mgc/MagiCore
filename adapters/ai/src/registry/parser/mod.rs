//! Export stable registry URL and model-ID parsing functions.
//! Export hàm phân tích URL registry và model ID ổn định.

mod parser;
pub use parser::{parse_hf_id, parse_onnx_path, parse_tfhub_url};
