/// One parallel command — một lệnh song song.
#[derive(Debug, Clone)]
pub struct ParallelTask {
    /// Program as invoked — chương trình được gọi.
    pub program: String,
    /// Arguments — tham số.
    pub args: Vec<String>,
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
