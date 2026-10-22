// Quantization failures — lỗi lượng tử hóa.
// Keep unsupported native quantization fail-closed — giữ chặn an toàn khi native quantize chưa có.

use std::fmt;

#[derive(Debug)]
pub(in crate::commands::model) enum QuantizeError {
    Unsupported,
}

impl fmt::Display for QuantizeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unsupported => f.write_str(
                "native GGUF quantization is not implemented; MagiCore will not invoke Python or an external quantizer"
            ),
        }
    }
}

impl std::error::Error for QuantizeError {}
