//! Declares static and native-PyPI AI capabilities.
//! Khai báo capability AI tĩnh và native PyPI.

use mgc_types::capabilities::Capability;

use super::super::AiAdapter;

impl AiAdapter {
    /// Capability manifest (Global Gate 1) — code-reality notes:
    /// - ProjectDetector: `detect_framework` (framework.rs) — real.
    /// - ScaffoldProvider: the `mgc create-ai` CLI scaffolder lane — real.
    /// - LifecycleRunner: the install lifecycle runs through the CLI
    ///   per-core lane (cli/src/commands/core/install/ai.rs) — real.
    /// - OptimizerProvider: `mgc optimizer` supports the ai core
    ///   (cli/src/commands/optimizer) — real.
    /// - AuditProvider: dependency scanners (pip-audit/cargo-audit/
    ///   govulncheck) + the model-artifact scanner — real.
    /// - CAPABILITIES_PYPI (pyproject.toml projects only):
    ///   DependencyResolver/LockfileProvider/ContentStoreProvider are real through the embedded native PyPI
    ///   engine; CLI add/remove/update edit the same MGC-owned manifest.
    ///
    /// Bảng capability (Global Gate 1) — ghi chú theo code thật;
    /// CAPABILITIES_PYPI (chỉ project pyproject.toml): resolve/lock/
    /// store THẬT qua engine PyPI native nhúng; CLI add/remove/update
    /// sửa manifest do MGC sở hữu. Virtualenv/interpreter không thuộc MGC.
    pub const CAPABILITIES: &'static [Capability] = &[
        Capability::ProjectDetector,
        Capability::ScaffoldProvider,
        Capability::LifecycleRunner,
        Capability::OptimizerProvider,
        Capability::AuditProvider,
    ];

    /// PyPI lane (pyproject.toml only): registry capabilities claimed
    /// because the embedded lib/python engine owns them natively.
    /// (Lane PyPI (chỉ pyproject.toml): claim registry vì engine
    /// lib/python nhúng sở hữu chúng native.)
    /// Union of the base and PYPI lanes — what `mgc capabilities` prints.
    ///
    /// The matrix describes the core, not one project shape; printing only the
    /// base omitted `dependency-resolver` while an AI project resolves through the
    /// embedded PYPI engine.
    /// (Hợp của base và PYPI — bản `mgc capabilities` in. Matrix mô tả core, không
    /// phải một dạng project.)
    pub const CAPABILITIES_ALL: &'static [Capability] = &[
        Capability::ProjectDetector,
        Capability::ScaffoldProvider,
        Capability::DependencyResolver,
        Capability::LockfileProvider,
        Capability::ArtifactFetcher,
        Capability::ContentStoreProvider,
        Capability::LifecycleRunner,
        Capability::OptimizerProvider,
        Capability::AuditProvider,
    ];

    pub const CAPABILITIES_PYPI: &'static [Capability] = &[
        Capability::ProjectDetector,
        Capability::ScaffoldProvider,
        Capability::DependencyResolver,
        Capability::LockfileProvider,
        Capability::ArtifactFetcher,
        Capability::ContentStoreProvider,
        Capability::LifecycleRunner,
        Capability::OptimizerProvider,
        Capability::AuditProvider,
    ];
}
