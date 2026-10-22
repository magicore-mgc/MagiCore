// Ecosystem plugin registry and global registration API.
// Registry plugin theo ecosystem và API đăng ký toàn cục.
use crate::Plugin;
use mgc_types::Ecosystem;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

/// Stores plugins by ecosystem for factory lookup.
/// Lưu plugin theo ecosystem để factory tra cứu.
#[derive(Default)]
pub struct PluginRegistry {
    by_ecosystem: Mutex<HashMap<&'static str, Plugin>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&self, plugin: Plugin) -> Result<(), String> {
        let key = plugin.ecosystem.as_str();
        // Poisoned lock means a previous holder panicked mid-insert —
        // fail the registration instead of panicking every later caller.
        // Lock bị poison khi holder trước panic giữa lúc ghi; từ chối đăng ký thay vì panic lặp.
        let mut map = self
            .by_ecosystem
            .lock()
            .map_err(|_| "plugin registry lock poisoned".to_string())?;
        if map.contains_key(key) {
            return Err(format!("plugin already registered for ecosystem '{key}'"));
        }
        map.insert(key, plugin);
        Ok(())
    }

    pub fn get(&self, ecosystem: Ecosystem) -> Option<Plugin> {
        // Poisoned → None: callers (factory) fall through to the built-in
        // adapters, which is the safe default. A poisoned global registry
        // is reported by the panicking holder's own crash evidence.
        // Khi lock poison, factory dùng adapter tích hợp; panic ban đầu vẫn có dấu vết riêng.
        self.by_ecosystem
            .lock()
            .ok()?
            .get(ecosystem.as_str())
            .cloned()
    }

    pub fn registered(&self) -> Vec<Ecosystem> {
        // Keys come only from `Ecosystem::as_str`, so `from_str` cannot
        // fail on them — but a foreign key must never panic listing.
        // (Key chỉ từ `as_str`; key lạ thì bỏ qua, không panic.)
        self.by_ecosystem
            .lock()
            .ok()
            .map(|map| {
                map.keys()
                    .filter_map(|key| Ecosystem::from_str(key))
                    .collect()
            })
            .unwrap_or_default()
    }
}
fn global_registry() -> &'static PluginRegistry {
    static REGISTRY: OnceLock<PluginRegistry> = OnceLock::new();
    REGISTRY.get_or_init(PluginRegistry::new)
}

/// Returns the global registry used by CLI dispatch instead of hard-coded matching.
/// Trả registry toàn cục để CLI tra cứu thay vì match cứng.
pub fn global() -> &'static PluginRegistry {
    global_registry()
}

/// Registers a plugin globally and returns an error when its ecosystem already exists.
/// Đăng ký plugin toàn cục; trả Err nếu ecosystem đã có, không tự ghi đè.
pub fn register(plugin: Plugin) -> Result<(), String> {
    global().register(plugin)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
