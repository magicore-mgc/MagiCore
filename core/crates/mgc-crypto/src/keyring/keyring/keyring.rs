//! Keyring state and public API — State và API public của keyring.
//! Delegate secure file work to storage owners — Giao xử lý file bảo mật cho storage owner.

use crate::keyring::KeyPair;
use crate::keyring::storage::{
    read_existing_for_backup, read_keyring_contents, reject_non_regular_existing_path,
    write_private_file_atomic,
};
use crate::{CryptoError, CryptoResult};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// Keyring for managing multiple keys — Keyring quản lý nhiều khóa
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keyring {
    /// All key pairs — Tất cả key pairs
    pub keys: Vec<KeyPair>,
    /// Default key ID — Key ID mặc định
    pub default_key_id: Option<String>,
}

impl Keyring {
    /// Create empty keyring — Tạo keyring rỗng
    pub fn new() -> Self {
        Keyring {
            keys: Vec::new(),
            default_key_id: None,
        }
    }

    /// Load keyring from file — Load keyring từ file
    pub fn load(path: &Path) -> CryptoResult<Self> {
        let content = read_keyring_contents(path)?;
        let keyring: Keyring = serde_json::from_str(&content)?;
        Ok(keyring)
    }

    /// Save keyring to file with secure permissions — Lưu keyring vào file với quyền bảo mật
    pub fn save(&self, path: &Path) -> CryptoResult<()> {
        self.save_impl(path)
    }

    /// Save after applying the same destination policy for every caller.
    /// Lưu sau khi áp dụng cùng policy đích cho mọi caller.
    fn save_impl(&self, path: &Path) -> CryptoResult<()> {
        // Validate the destination through the path-policy owner — kiểm tra path qua owner policy.
        super::super::storage::validate_keyring_path(path)?;

        // Create parent directory if not exists — Tạo thư mục cha nếu chưa có
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = serde_json::to_string_pretty(self)?;
        let previous = read_existing_for_backup(path)?;
        if let Some(previous) = previous {
            let backup = path.with_extension("json.bak");
            reject_non_regular_existing_path(&backup)?;
            write_private_file_atomic(&backup, previous.as_bytes())?;
        }

        reject_non_regular_existing_path(path)?;
        write_private_file_atomic(path, content.as_bytes())
    }

    /// Add new key pair — Thêm key pair mới
    pub fn add_key(&mut self, key_pair: KeyPair) {
        // Set as default if this is the first key — Đặt làm mặc định nếu là khóa đầu tiên
        if self.keys.is_empty() {
            self.default_key_id = Some(key_pair.key_id.clone());
        }
        self.keys.push(key_pair);
    }

    /// Get default key — Lấy khóa mặc định
    pub fn default_key(&self) -> Option<&KeyPair> {
        let key_id = self.default_key_id.as_ref()?;
        self.get_key(key_id)
    }

    /// Get key by ID — Lấy khóa theo ID
    pub fn get_key(&self, key_id: &str) -> Option<&KeyPair> {
        self.keys.iter().find(|k| k.key_id == key_id)
    }

    /// Set default key — Đặt khóa mặc định
    pub fn set_default(&mut self, key_id: &str) -> CryptoResult<()> {
        if !self.keys.iter().any(|k| k.key_id == key_id) {
            return Err(super::super::error::KeyringError::KeyNotFound(key_id.to_string()).into());
        }
        self.default_key_id = Some(key_id.to_string());
        Ok(())
    }

    /// Get default keyring path — Lấy đường dẫn keyring mặc định.
    /// Fails closed without a home directory instead of panicking trust commands.
    /// Từ chối rõ khi thiếu home thay vì panic trong lệnh trust.
    pub fn default_path() -> CryptoResult<PathBuf> {
        dirs::home_dir()
            .ok_or_else(|| {
                CryptoError::from(super::super::error::KeyringError::HomeDirectoryMissing)
            })
            .map(|home| home.join(".magicore").join("keys").join("keyring.json"))
    }

    /// Initialize keyring with new key if not exists
    /// Khởi tạo keyring với khóa mới nếu chưa có
    pub fn init_if_not_exists() -> CryptoResult<Self> {
        let path = Self::default_path()?;
        if path.exists() {
            Self::load(&path)
        } else {
            let mut keyring = Self::new();
            let key_pair = KeyPair::generate()?;
            keyring.add_key(key_pair);
            keyring.save(&path)?;
            Ok(keyring)
        }
    }
}
impl Default for Keyring {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[path = "test/keyring_save.rs"]
mod keyring_save;
