mod python_compatibility;
// `abi_compatible` stays crate-private: it is the GIL-flavor shorthand used by
// unit tests, while production selection speaks `InterpreterAbiFlavor`.
// (`abi_compatible` chỉ dùng nội bộ cho test; production dùng InterpreterAbiFlavor.)
#[cfg(test)]
pub(in crate::protocols::pypi) use python_compatibility::detected_interpreter_python;
pub(in crate::protocols::pypi) use python_compatibility::{
    InterpreterAbiFlavor, abi_compatible_for_interpreter, consumer_python, requires_python_allows,
};
