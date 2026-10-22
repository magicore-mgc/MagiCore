mod python_compatibility;
pub(in crate::protocols::pypi) use python_compatibility::{
    abi_compatible, consumer_python, requires_python_allows,
};
