# AI Adapter (mgc-ai-adapter)

<!-- English module guide: purpose, source map, and review boundaries. -->
<!-- Hướng dẫn tiếng Việt về chức năng và ranh giới review của module. -->

## Purpose

Adapts AI projects to MagiCore core, capability, audit, cache, and install interfaces. The detector recognizes the explicit `python-agent` and `mcp-server` labels; model downloads/checksums and a native Hugging Face client are also present. This does not claim universal AI-language or model-runtime support.

## Source map

- [`adapter.rs`](../src/adapter/adapter/)
- [`framework.rs`](../src/framework/)
- [`install/mod.rs`](../src/install/mod.rs)
- [Native Hugging Face client owners](../src/native/hf_client/README.md)

## Handoff and review

Trace the selected capability before changing dependency/runtime behavior. Preserve explicit unsupported handling and checksum verification; framework detection alone does not prove a complete lane.

## Verification

After code changes, run this crate suite from the repository root with `cargo test -p mgc-ai-adapter`. For documentation-only changes, check links, language, and Git ignore behavior instead of running unrelated workspace tests.
