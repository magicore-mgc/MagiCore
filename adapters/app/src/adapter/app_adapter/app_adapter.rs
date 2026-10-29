//! Public AppAdapter state, constructor and language capability sets.
//! State AppAdapter public, constructor và capability set theo ngôn ngữ.

use crate::adapter::PendingInstallState;
use crate::language::AppLanguage;
use mgc_types::capabilities::Capability;
use std::path::PathBuf;

pub struct AppAdapter {
    pub language: AppLanguage,
    // Root captured at detection for project-specific resolver configuration.
    // (Giữ gốc project để dùng cấu hình resolver theo project.)
    pub project_root: PathBuf,
    // Carries native resolution locks and the Swift protocol cache into install.
    // (Chuyển lock resolve và cache protocol Swift sang bước install.)
    pub(in crate::adapter) pending_install: PendingInstallState,
}

impl AppAdapter {
    /// Public constructor for direct-language use (tests, CLI lanes) —
    /// the pending lock starts empty and the project root is unknown.
    /// (Constructor public cho dùng trực tiếp theo ngôn ngữ (test, lane
    /// CLI) — pending lock khởi tạo rỗng và gốc project không biết.)
    pub fn new(language: AppLanguage) -> Self {
        Self {
            language,
            project_root: PathBuf::new(),
            pending_install: PendingInstallState::default(),
        }
    }

    /// Capability manifest (Global Gate 1) — code-reality notes:
    /// per-language truth, never a core-level blanket (a blanket
    /// DependencyResolver claim overclaims for Kotlin/ObjC, which fail
    /// closed). `CAPABILITIES` (used by `mgc capabilities`) is the BASE
    /// set every language shares; the instance method below adds
    /// DependencyResolver exactly for Flutter/Swift, and the
    /// capabilities test enforces both directions.
    pub const CAPABILITIES: &'static [Capability] = Self::CAPABILITIES_BASE;

    pub(in crate::adapter) const CAPABILITIES_BASE: &'static [Capability] = &[
        Capability::ProjectDetector,
        Capability::ScaffoldProvider,
        Capability::AuditProvider,
        // `mgc optimizer` dispatches every core through the shared generators,
        // including this one — the same evidence `ai` declared it with.
        // (`mgc optimizer` dispatch mọi core qua generator dùng chung, kể cả core
        // này — cùng dẫn chứng `ai` dùng.)
        Capability::OptimizerProvider,
    ];
    pub(in crate::adapter) const CAPABILITIES_RESOLVER: &'static [Capability] = &[
        Capability::ProjectDetector,
        Capability::ScaffoldProvider,
        Capability::ContentStoreProvider,
        Capability::LockfileProvider,
        Capability::AuditProvider,
        Capability::DependencyResolver,
        // Same shared optimizer dispatch as the base set.
        Capability::OptimizerProvider,
    ];
}
