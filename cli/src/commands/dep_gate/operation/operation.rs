//! Dependency operation vocabulary — vocabulary operation dependency.
//! Canonical operation enum and names shared by gate callers and capability output.
//! Enum và tên chuẩn dùng chung giữa gate và capability.

/// Dependency-lifecycle operations covered by the firewall: the five
/// user verbs plus the native pipeline stages a real package manager
/// owns (resolve → lock → fetch → verify → store → materialize, plus
/// frozen/offline reinstalls and store GC/recovery).
/// (Các operation lifecycle dependency tường lửa phủ: năm động từ user
/// cộng các giai đoạn pipeline PM native sở hữu.)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DepOp {
    Install,
    Add,
    Remove,
    Update,
    List,
    Resolve,
    Lock,
    Fetch,
    Verify,
    Store,
    Materialize,
    FrozenInstall,
    OfflineReinstall,
    Gc,
}

impl DepOp {
    /// Canonical operation inventory shared by every capability surface.
    /// Danh sách operation chuẩn dùng chung cho mọi bề mặt capability.
    pub const ALL: &'static [DepOp] = &[
        DepOp::Install,
        DepOp::Add,
        DepOp::Remove,
        DepOp::Update,
        DepOp::List,
        DepOp::Resolve,
        DepOp::Lock,
        DepOp::Fetch,
        DepOp::Verify,
        DepOp::Store,
        DepOp::Materialize,
        DepOp::FrozenInstall,
        DepOp::OfflineReinstall,
        DepOp::Gc,
    ];

    /// CLI-canonical operation name — matches user-facing verbs; pipeline
    /// stages use their canonical stage names.
    /// Tên operation chuẩn CLI — khớp động từ user gọi.
    pub fn as_str(&self) -> &'static str {
        match self {
            DepOp::Install => "install",
            DepOp::Add => "add",
            DepOp::Remove => "remove",
            DepOp::Update => "update",
            DepOp::List => "list",
            DepOp::Resolve => "resolve",
            DepOp::Lock => "lock",
            DepOp::Fetch => "fetch",
            DepOp::Verify => "verify",
            DepOp::Store => "store",
            DepOp::Materialize => "materialize",
            DepOp::FrozenInstall => "frozen-install",
            DepOp::OfflineReinstall => "offline-reinstall",
            DepOp::Gc => "gc",
        }
    }
}
