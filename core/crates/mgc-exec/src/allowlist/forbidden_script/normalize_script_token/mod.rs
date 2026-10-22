#[path = "normalize_script_token.rs"]
mod normalize_script_token;

pub(crate) use normalize_script_token::normalize_script_token;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
