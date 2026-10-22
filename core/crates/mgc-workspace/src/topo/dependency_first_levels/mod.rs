// Dependency-first level wiring — expose a separate execution-order contract.
// Wiring level phụ thuộc trước — cung cấp contract riêng cho execution.
mod dependency_first_levels;
#[cfg(test)]
mod test;

pub use dependency_first_levels::dependency_first_levels;
