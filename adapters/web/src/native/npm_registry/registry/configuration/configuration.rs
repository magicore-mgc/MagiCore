//! registry configuration.
// Chứa đúng trách nhiệm của module; giữ nguyên hành vi, API, chính sách lỗi và bảo mật hiện có.

use crate::native::npm_registry::{NpmRegistry, ScopedRegistryClient};
use anyhow::Result;

impl NpmRegistry {
    /// Preserve typed credential scopes across every registry request.
    /// Giữ scope theo instance, không suy ngược quyền từ token hoặc URL artifact.
    pub fn new_with_scoped_chain(entries: Vec<mgc_config::chain::ScopedRegistry>) -> Result<Self> {
        let primary = entries
            .first()
            .ok_or_else(|| anyhow::anyhow!("no registry configured"))?;
        for (index, entry) in entries.iter().enumerate() {
            if entries.iter().skip(index + 1).any(|other| {
                entry.registry.url == other.registry.url
                    || (entry.credential_scope.is_some()
                        && entry.credential_scope == other.credential_scope
                        && entry.registry.token != other.registry.token)
            }) {
                return Err(anyhow::anyhow!("ambiguous registry credential chain"));
            }
        }
        let mut registry = Self::new_with_chain(
            &primary.registry.url,
            primary.registry.token.clone(),
            entries
                .iter()
                .skip(1)
                .map(|entry| (entry.registry.url.clone(), entry.registry.token.clone()))
                .collect(),
        );
        registry.scoped_clients = Some(
            entries
                .iter()
                .map(|entry| {
                    Ok((
                        entry.registry.url.clone(),
                        ScopedRegistryClient::new(entry)?,
                    ))
                })
                .collect::<Result<Vec<_>>>()?,
        );
        Ok(registry)
    }

    /// Download tasks retain immutable credential scopes and reuse connection pools.
    /// Không tái tạo từ token scalar; clone giữ policy theo instance của adapter.
    pub fn clone_for_downloads(&self) -> Self {
        Self {
            registry_url: self.registry_url.clone(),
            token: self.token.clone(),
            fallbacks: self.fallbacks.clone(),
            scoped_clients: self.scoped_clients.clone(),
            age_armed: std::sync::atomic::AtomicBool::new(self.age_gate_armed()),
        }
    }

    /// Arm/disarm full-packument fetch mode for this client (called by
    /// the owning provider when it (re-)arms its age policy).
    /// (Bật/tắt chế độ fetch full doc cho client này.)
    pub fn set_age_gate_armed(&self, armed: bool) {
        self.age_armed
            .store(armed, std::sync::atomic::Ordering::SeqCst);
    }

    /// Full packuments when armed (abbreviated docs omit `time`).
    /// (Full doc khi cổng tuổi bật.)
    pub fn age_gate_armed(&self) -> bool {
        self.age_armed.load(std::sync::atomic::Ordering::SeqCst)
    }
}
