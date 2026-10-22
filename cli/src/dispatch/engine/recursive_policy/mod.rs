// Recursive policy wiring — export the classifier and refusal functions to engine.
// Wiring recursive policy — export classifier và hàm từ chối cho engine.
mod recursive_policy;
#[cfg(test)]
mod test;

pub(super) use recursive_policy::{
    recursive_supported, reject_filter_without_recursive, reject_unsupported_filter,
    reject_unsupported_recursive,
};
