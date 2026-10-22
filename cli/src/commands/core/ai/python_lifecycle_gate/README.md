# AI native-Python lifecycle gate

Owns the AI command check that admits only projects supported by the native Python dependency lifecycle. It delegates the decision to `mgc_ai_adapter::uses_native_python_lane` and reports the existing typed AI error when unsupported; it never starts pip, uv, or another external manager.

# Gate lifecycle Python native của AI

Sở hữu check lệnh AI chỉ nhận project có lifecycle dependency Python native được hỗ trợ. Quyết định vẫn do `mgc_ai_adapter::uses_native_python_lane`; lane chưa hỗ trợ trả typed error AI hiện tại và không gọi pip, uv hay package manager khác.
