use crate::policy_manifest::PolicyManifest;

/// Find the exact exception for an invocation in a verified manifest.
/// Returns `None` unless program, full args, framework, version, OS and
/// operation all match one entry (Spec rev4 §5/§7 profile binding is
/// enforced by the caller through [`crate::policy::resolve`]).
/// (Tìm exception chính xác cho invocation trong manifest đã xác minh.
/// Trả `None` trừ khi program, đủ args, framework, version, OS và operation
/// đều khớp một entry (ràng buộc profile ở Spec rev4 §5/§7 do caller thực
/// thi qua [`crate::policy::resolve`]).)
pub fn find_exception(
    manifest: &PolicyManifest,
    program: &str,
    args: &[String],
    framework: &str,
    framework_version: &str,
    os: &str,
    operation: &str,
) -> Option<crate::policy::ReadOnlyException> {
    let tool = crate::policy::tool_basename(program);
    manifest
        .entries
        .iter()
        .find(|entry| {
            entry.program == tool
                && entry.args_exact == args
                && entry.framework == framework
                && entry.framework_version == framework_version
                && entry.os == os
                && entry.operation == operation
        })
        .map(|entry| crate::policy::ReadOnlyException {
            program: entry.program.clone(),
            args_exact: entry.args_exact.clone(),
            unexpired: true,
            answer: None,
        })
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
