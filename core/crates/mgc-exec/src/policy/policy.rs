use super::{
    DenyCode, Invocation, LaneProfile, PolicyDecision, PolicyMode, ReadOnlyException,
    is_mutation_invocation, is_read_only_probe, profile_allows_invocation,
    profile_invocation::invocation_key, tool_basename,
};
use crate::allowlist::FORBIDDEN_TOOLS;

/// Resolve one invocation to a typed decision (pure function, no I/O).
/// Phân giải một invocation thành quyết định typed (hàm thuần, không I/O).
///
/// Precedence (Spec rev4 §7): mutation-deny first — profile, exception and
/// local config never turn mutation into allow in native mode.
/// (Ưu tiên (Spec rev4 §7): mutation-deny trước — profile, exception và
/// config local không bao giờ biến mutation thành cho phép ở native mode.)
pub fn resolve(
    mode: &PolicyMode,
    invocation: &Invocation,
    profile: Option<&LaneProfile>,
    exception: Option<&ReadOnlyException>,
) -> PolicyDecision {
    let mutation = is_mutation_invocation(&invocation.program, &invocation.args, &invocation.env);

    // Row 4: mutation-deny wins over everything in native mode.
    // (Hàng 4: mutation-deny thắng mọi thứ ở native mode.)
    if mutation {
        match mode {
            PolicyMode::Native => {
                return PolicyDecision::Deny {
                    code: DenyCode::NativeMutationDeny,
                };
            }
            PolicyMode::CompatRunner { manager } => {
                if invocation.in_compat_subtree {
                    return PolicyDecision::CompatAudit {
                        note: format!(
                            "mutation inside declared '{manager}' compat subtree: {}",
                            invocation_key(&invocation.program, &invocation.args)
                        ),
                    };
                }
                return PolicyDecision::Deny {
                    code: DenyCode::NativeMutationDeny,
                };
            }
        }
    }

    // Compat subtree: non-mutation work runs with warning + audit.
    // (Subtree compat: việc không mutation chạy kèm cảnh báo + audit.)
    if let PolicyMode::CompatRunner { manager } = mode
        && invocation.in_compat_subtree
    {
        return PolicyDecision::CompatAudit {
            note: format!(
                "execution inside declared '{manager}' compat subtree: {}",
                invocation_key(&invocation.program, &invocation.args)
            ),
        };
    }

    // Row 5: exact read-only exception (unexpired; profile-bound on profiled lanes).
    // (Hàng 5: exception đọc chính xác (còn hạn; gắn profile ở lane đã profile).)
    if let Some(exc) = exception
        && tool_basename(&invocation.program) == exc.program
        && invocation.args == exc.args_exact
        && exc.unexpired
        && is_read_only_probe(&invocation.program, &invocation.args)
        && profile.is_some_and(|lane| {
            profile_allows_invocation(lane, &invocation.program, &invocation.args)
        })
    {
        if let Some(answer) = &exc.answer {
            return PolicyDecision::SelfAnswer(answer.clone());
        }
        return PolicyDecision::Allow;
    }

    let tool = tool_basename(&invocation.program);
    if FORBIDDEN_TOOLS.contains(&tool.as_str()) {
        // Row 6: profiled lane outside the active profile → deny.
        // (Hàng 6: lane đã profile mà ngoài profile hiệu lực → deny.)
        if let Some(lane) = profile {
            let in_profile = profile_allows_invocation(lane, &invocation.program, &invocation.args);
            // In-profile but without a valid exception: the exception path
            // failed (missing or expired) — deny with the row-5 code.
            // (Trong profile nhưng không có exception hợp lệ: đường exception
            // hỏng (thiếu/hết hạn) — deny bằng mã hàng 5.)
            if in_profile {
                return PolicyDecision::Deny {
                    code: DenyCode::MissingException,
                };
            }
            return PolicyDecision::Deny {
                code: DenyCode::OutsideActiveProfile,
            };
        }
        // Unprofiled lane: forbidden name without an exact exception → deny.
        // (Lane chưa profile: tên cấm mà không có exception chính xác → deny.)
        return PolicyDecision::Deny {
            code: DenyCode::ForbiddenNameNoException,
        };
    }

    // Ordinary descendant: runs with observability audit (not a safety proof).
    // (Descendant thường: chạy kèm audit quan sát (không phải chứng minh an toàn).)
    PolicyDecision::Allow
}

#[cfg(test)]
#[path = "test/policy.rs"]
mod tests;
