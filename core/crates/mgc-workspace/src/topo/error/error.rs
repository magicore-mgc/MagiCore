//! Public topological-order failure contract.
//! Hợp đồng lỗi public của sắp thứ tự topo.

#[derive(Debug, PartialEq, Eq)]
pub enum TopoError {
    Cycle(Vec<String>),
}

impl std::fmt::Display for TopoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TopoError::Cycle(path) => {
                write!(f, "workspace dependency cycle: {}", path.join(" -> "))
            }
        }
    }
}

impl std::error::Error for TopoError {}
