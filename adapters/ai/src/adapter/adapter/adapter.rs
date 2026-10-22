//! Constructs and selects the AI project adapter.
//! Khởi tạo và chọn adapter cho project AI.

use crate::framework::{AiFramework, detect_framework};
use std::path::Path;

pub struct AiAdapter {
    pub framework: AiFramework,
    /// Native PyPI lane — present ONLY for pyproject.toml projects, where
    /// python deps ride the lib/python engine for resolve/install
    /// (resolve → verified fetch → CAS → mgc.lock, zero uv/pip spawn).
    /// Manifest IO + ownership predicate bypass it and call
    /// `mgc-python-service` directly (S09b/D012); the lane stays until the
    /// shared service owns the resolve/install engine (P04 wave).
    /// Existing uv.lock/requirements.lock projects have NO lane here until
    /// the user migrates lock ownership.
    /// (Lane PyPI native chỉ bật khi không chiếm lockfile toolchain;
    /// manifest IO/predicate gọi thẳng service dùng chung, resolve/install
    /// vẫn qua engine lib nhúng.)
    pub python_lane: Option<mgc_lib_adapter::LibAdapter>,
}
/// Select the native PyPI lane only when MGC can own the lock lifecycle
/// without taking over an existing uv/pip lockfile. A project with an
/// existing foreign lock needs an explicit migration before ownership
/// changes. (Chỉ chọn lane PyPI native khi không chiếm lockfile toolchain.)
/// Ownership predicate is owned by the shared Python service (S09b/D012) —
/// called directly, no longer through the lib facade.
/// (Predicate thuộc service dùng chung — gọi trực tiếp, không qua facade lib.)
pub fn uses_native_python_lane(root: &Path) -> bool {
    mgc_python_service::manifest::supports_native_python_project(root)
}
pub fn adapter_for(root: &Path) -> Option<AiAdapter> {
    let framework = detect_framework(root)?;
    // The lib python engine constructor is infallible for non-TS
    // languages (no web engine is built) — an error here is a logic bug,
    // so fail loudly rather than silently dropping the native engine.
    // (Constructor engine python của lib không thể lỗi với ngôn ngữ
    // non-TS — lỗi ở đây là bug logic, fail to rather than bỏ im lặng.)
    // S09b/D012: `adapter_for_ai_python` is a lib-owned COMPAT facade over
    // the shared Python service's ownership scope (`security_core = "ai"`,
    // age-gate policy). It stays until the resolve/install engine moves to
    // the shared service (P04 wave) — TODO(S09b-followup): drop the
    // `mgc-lib-adapter` dep with that move. Manifest IO already bypasses it.
    // (`adapter_for_ai_python` là facade tương thích của lib — giữ tới khi
    // engine chuyển sang service dùng chung.)
    let python_lane = if uses_native_python_lane(root) {
        Some(
            mgc_lib_adapter::adapter_for_ai_python(root)
                .expect("lib python engine construction is infallible"),
        )
    } else {
        None
    };
    Some(AiAdapter {
        framework,
        python_lane,
    })
}
