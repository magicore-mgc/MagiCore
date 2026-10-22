//! Process-local in-memory cache facade. / Facade cache RAM theo process.

mod cache_daemon;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

pub use cache_daemon::MemoryCacheDaemon;
