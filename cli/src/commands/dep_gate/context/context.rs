//! Full dependency gate context — context đầy đủ cho dependency gate.
//! Contains detected project identity and operation evidence required by policy.
//! Chứa identity dự án và operation đã detect để policy quyết định.

use crate::commands::dep_gate::DepOp;

/// Full dependency-operation context — EVERY dependency lane MUST build
/// one before calling `gate()`. Ownership can additionally require a
/// concrete manifest format; an undetermined ecosystem/format fails closed.
/// (Context đầy đủ cho operation dependency — MỌI lane dependency PHẢI
/// dựng một cái trước khi gọi `gate()`. Không có dạng gọn bỏ-ngôn-ngữ:
/// ecosystem không xác định được thì fail Unsupported, không fallback
/// sang lane delegated chung.)
#[derive(Debug, Clone, Copy)]
pub struct DepContext<'a> {
    /// CLI core: web | lib | ai | app | game | iot | clo | cicd | hardware.
    /// Core CLI mà context đại diện.
    pub core: &'a str,
    /// Language/ecosystem id (canonical `eco::*` strings): the lane's
    /// DETECTED project ecosystem. `None`/unknown ⇒ Unsupported.
    /// (Ecosystem project lane DETECT được. Không có/không rõ ⇒ Unsupported.)
    pub ecosystem: Option<&'a str>,
    /// Framework id where the lane knows one (iot framework, else None).
    /// Framework của lane khi nhận diện được, chủ yếu cho IoT.
    pub framework: Option<&'a str>,
    /// Deploy/materialize target where the lane knows one (iot board).
    /// Target deploy/materialize khi lane biết rõ, ví dụ board IoT.
    pub target: Option<&'a str>,
    /// Concrete dependency manifest format where ownership differs within
    /// one ecosystem (e.g. Java Maven POM vs executable Gradle scripts).
    /// None means the format has not been proven; the gate fails closed.
    /// Manifest cụ thể; thiếu bằng chứng thì gate từ chối.
    pub manifest_format: Option<&'a str>,
    /// The invoked operation (verb or pipeline stage).
    /// Verb người dùng gọi hoặc stage pipeline nội bộ.
    pub op: DepOp,
}

impl<'a> DepContext<'a> {
    /// Build the mandatory gate context for one dependency operation.
    /// (Dựng context gate bắt buộc cho một operation dependency.)
    pub fn new(
        core: &'a str,
        ecosystem: Option<&'a str>,
        framework: Option<&'a str>,
        target: Option<&'a str>,
        op: DepOp,
    ) -> Self {
        Self {
            core,
            ecosystem,
            framework,
            target,
            manifest_format: None,
            op,
        }
    }

    /// Attach a manifest format detected from the project root.
    /// Gắn format manifest đã detect từ project root.
    pub fn with_manifest_format(mut self, manifest_format: Option<&'a str>) -> Self {
        self.manifest_format = manifest_format;
        self
    }

    /// Human/gate-log lane description: `core` plus the detected
    /// ecosystem, framework, target and manifest format when known
    /// (`iot[esp32-rust|board:esp32]`-style precision for audit lines and
    /// user-facing messages).
    /// (Mô tả lane cho log: core kèm ecosystem/framework/target/manifest khi biết.)
    pub fn describe(&self) -> String {
        let mut out = self.core.to_string();
        if let Some(eco) = self.ecosystem {
            out.push_str(&format!("[{eco}"));
            if let Some(fw) = self.framework {
                out.push_str(&format!("|{fw}"));
            }
            if let Some(target) = self.target {
                out.push_str(&format!("|target:{target}"));
            }
            if let Some(format) = self.manifest_format {
                out.push_str(&format!("|manifest:{format}"));
            }
            out.push(']');
        }
        out
    }
}
