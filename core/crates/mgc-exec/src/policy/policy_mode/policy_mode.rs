/// Policy mode for one invocation — chế độ policy cho một lần gọi.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyMode {
    /// Native MagiCore engine — the default, no delegation.
    /// Engine native MagiCore — mặc định, không ủy quyền.
    Native,
    /// Explicit compat runner subtree (e.g. `--compat-runner npm`).
    /// Cây compat tường minh (vd `--compat-runner npm`).
    CompatRunner {
        /// Declared manager name, lowercase basename (e.g. `"npm"`).
        /// Tên manager đã khai báo, basename lowercase (vd `"npm"`).
        manager: String,
    },
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
