#[path = "invocation.rs"]
mod invocation;

pub use invocation::ScriptInvocation;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
