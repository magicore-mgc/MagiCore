/// Closed set of read-only Git queries used by publish integrity checks.
/// Tập đóng các truy vấn Git chỉ đọc dùng cho kiểm tra tính toàn vẹn publish.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::commands) enum GitReadQuery {
    ValidateBranch(String),
    WorkingTreeStatus,
    CurrentBranch,
    UpstreamRef(String),
    CommitsBehind(String),
}

impl GitReadQuery {
    pub(in crate::commands::publish) fn argv(&self) -> Vec<String> {
        match self {
            Self::ValidateBranch(branch) => {
                vec!["check-ref-format".into(), format!("refs/heads/{branch}")]
            }
            Self::WorkingTreeStatus => vec!["status".into(), "--porcelain".into()],
            Self::CurrentBranch => vec![
                "rev-parse".into(),
                "--verify".into(),
                "--abbrev-ref".into(),
                "--end-of-options".into(),
                "HEAD".into(),
            ],
            Self::UpstreamRef(branch) => vec![
                "rev-parse".into(),
                "--verify".into(),
                "--abbrev-ref".into(),
                "--symbolic-full-name".into(),
                "--end-of-options".into(),
                format!("{branch}@{{upstream}}"),
            ],
            Self::CommitsBehind(branch) => vec![
                "rev-list".into(),
                "--count".into(),
                "--end-of-options".into(),
                format!("{branch}..{branch}@{{upstream}}"),
            ],
        }
    }
}
