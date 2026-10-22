/// One process invocation to judge — một lần gọi process cần phán quyết.
#[derive(Debug, Clone)]
pub struct Invocation {
    /// Program as invoked (basename or path) — chương trình được gọi.
    pub program: String,
    /// Arguments as invoked — tham số được gọi.
    pub args: Vec<String>,
    /// Explicit env for flag/env-gated verbs (GOPROXY, `--offline`...).
    /// Env tường minh cho verb phụ thuộc flag/env (GOPROXY, `--offline`...).
    pub env: Vec<(String, String)>,
    /// True when spawned under the declared compat runner.
    /// Đúng khi được spawn dưới runner compat đã khai báo.
    pub in_compat_subtree: bool,
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
