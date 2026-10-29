# MagiCore Adapters — Capability Source of Truth
# Năng lực adapter — nguồn sự thật duy nhất

> **Do not hand-maintain this file.** The authoritative manifest is what each
> adapter declares in `CAPABILITIES`; the CLI prints it:
>
> ```bash
> mgc capabilities          # JSON, one entry per core
> ```
>
> A file that restated the matrix by hand drifted from the code once already
> (the previous revision, generated 2026-08-26, still described every adapter as
> "structure + stubs" while `adapters/app` had grown to 11k lines).
> (Không tự duy trì file này. Nguồn sự thật là `CAPABILITIES` từng adapter khai
> báo và `mgc capabilities` in ra. Bản trước sinh 2026-08-26 vẫn mô tả mọi
> adapter là "stub" trong khi `adapters/app` đã 11k dòng — đúng lỗi cần tránh.)

## Where the truth lives — Nơi sự thật nằm

| Owner | Declaration |
|---|---|
| `adapters/web` | `src/adapter/adapter.rs` → `WebAdapter::CAPABILITIES` |
| `adapters/lib` | `src/adapter/package_adapter/capability_profile/capability_profile.rs` (`CAPABILITIES`, `_TS`, `_JAVA_DELEGATED`, `_PYTHON_UNOWNED`) |
| `adapters/ai` | `src/adapter/adapter/capability_manifest/capability_manifest.rs` |
| `adapters/app` | `src/adapter/app_adapter/app_adapter.rs` |
| `adapters/game` | `src/adapter/capability_manifest/capability_manifest.rs` |
| `adapters/iot` | `src/adapter/adapter.rs` |
| `adapters/cloud` | `src/adapter/adapter.rs` |
| `adapters/cicd` | `src/adapter/capability_manifest/capability_manifest.rs` |
| `adapters/hardware` | `src/adapter/capability_manifest/capability_manifest.rs` |

`mgc capabilities` merges them through
`cli/src/commands/capabilities/core_catalog/core_catalog.rs`; every core name in
that catalog must resolve, and the unknown-core branch is an error rather than an
empty list.

## Rules an adapter must satisfy — Quy tắc adapter phải giữ

1. **Declare only what is implemented.** A capability in `CAPABILITIES` without a
   real override is a lie the whole product inherits: `mgc capabilities` feeds
   the lifecycle matrix and the release gate.
   (Chỉ khai cái đã triển khai — capability khai mà không có implementation thật
   sẽ lan ra toàn sản phẩm vì matrix và release gate đọc chính nó.)
2. **Unclaimed capabilities fail closed, never silently succeed.** The default
   implementations in `mgc_types::capabilities` return
   `MgError::Unsupported { guidance }`. An adapter must not add a no-op override
   to "make the command work".
   (Capability không khai fail-closed, không bao giờ trả success im lặng.)
3. **A narrower language selects a narrower manifest.** `adapters/lib` proves the
   pattern: `Ts` claims the full set, `Java` with a Gradle build delegates, and
   Python without a native owner claims less. Splitting by language is correct;
   claiming one language's capability for another is not.
   (Ngôn ngữ hẹp chọn manifest hẹp — tách theo ngôn ngữ là đúng, mượn năng lực
   của ngôn ngữ khác là sai.)
4. **Tests live beside the declaration.** Each manifest's `test/` folder asserts
   that the list matches the overrides that exist.
   (Test nằm cạnh khai báo.)
