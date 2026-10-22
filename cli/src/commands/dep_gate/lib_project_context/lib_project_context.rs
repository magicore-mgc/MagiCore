//! Build gate context from the Lib adapter's project probes — dựng context từ probe adapter Lib.
//! Uses the same language and concrete manifest detection as the library adapter.
//! Dùng cùng phát hiện ngôn ngữ và manifest cụ thể với adapter Lib.

use crate::commands::dep_gate::{DepContext, DepOp};

/// Build the library-core gate context from the same root manifest probes
/// used by the lib adapter. This prevents language-only capability claims
/// from admitting unsupported manifest formats.
/// Tạo context gate lib từ manifest thực tế, tránh capability chỉ dựa vào
/// ngôn ngữ mà nhận nhầm format chưa được hỗ trợ.
pub fn lib_project_context(root: &std::path::Path, op: DepOp) -> DepContext<'static> {
    #[cfg(any(feature = "lib", feature = "ai"))]
    {
        let language = mgc_lib_adapter::detect_language(root);
        let ecosystem = language.map(|language| language.ecosystem());
        let manifest_format = language
            .and_then(|language| mgc_lib_adapter::dependency_manifest_format(root, language));
        DepContext::new("lib", ecosystem, None, None, op).with_manifest_format(manifest_format)
    }
    #[cfg(not(any(feature = "lib", feature = "ai")))]
    {
        // A per-core binary without Lib/AI must never advertise a lane it
        // cannot execute; an unknown ecosystem fails closed in owner_for.
        // Binary chỉ bật core khác không được quảng bá lane Lib không thể chạy.
        let _ = root;
        DepContext::new("lib", None, None, None, op)
    }
}
