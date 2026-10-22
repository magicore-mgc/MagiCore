# mgc-types

<!-- English module guide: purpose, source map, and review boundaries. -->
<!-- Hướng dẫn tiếng Việt về chức năng và ranh giới review của module. -->

## Purpose

Defines shared contracts and domain types: adapters/capabilities, ecosystems, package/version identity, manifests, publish/patch models, errors/results, JSONC, and a legacy literal-string censor helper. The helper does not redact configuration values and is not a security-grade redactor. Public changes may affect most crates and require caller plus serialization/API compatibility review.

## Source map

- [`lib.rs`](../src/lib.rs)
- [`adapter/`](../src/adapter/README.md)
- [`capabilities/`](../src/capabilities/README.md)
- [`ecosystem/`](../src/ecosystem/README.md) — shared enum, string mapping and nearest tests.
- [`error/`](../src/error/README.md)
- [`jsonc/`](../src/jsonc/README.md)
- [`manifest/`](../src/manifest/README.md) — shared manifest model and nearest behavior tests.
- [`package/`](../src/package/README.md)
- [`patch/`](../src/patch/README.md) — config-time patch specs and lockfile patch records.
- [`publish/`](../src/publish/README.md) — publish request and result contracts with nearest tests.
- [`version/`](../src/version/README.md)
- [`secret_censor/`](../src/secret_censor/README.md) — legacy literal helper; not a secure value redactor.

## Handoff and review

Analyze callers before changing public traits/enums or identity semantics. Check serialization, equality/order/hash, ecosystem ownership strings, and core parity.

## Verification

After code changes, run this crate suite from the repository root with `cargo test -p mgc-types`. For documentation-only changes, check links, language, and Git ignore behavior instead of running unrelated workspace tests.
