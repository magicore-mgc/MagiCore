# Runtime profile tests / Test profile runtime

`runtime.rs` tests preserve the runtime names and verify every ordered flag/permission profile. The test bodies are separate from production code and are included only under `cfg(test)`.
