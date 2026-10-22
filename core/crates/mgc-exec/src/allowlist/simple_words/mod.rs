#[path = "simple_words.rs"]
mod simple_words;

pub(super) use simple_words::split_simple_words;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
