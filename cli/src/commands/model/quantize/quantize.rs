// quantize owner — quantize owner implementation.
// Responsibility boundary — ranh giới trách nhiệm của owner.
use super::QuantizeError;
use anyhow::Result;
/// Fail closed until MagiCore ships its own GGUF quantization engine.
/// Fail-closed cho tới khi MagiCore có engine lượng tử hóa GGUF riêng.
pub(in crate::commands::model) fn quantize(
    _path: &str,
    _target: &str,
    _output: Option<&str>,
) -> Result<()> {
    Err(QuantizeError::Unsupported.into())
}
