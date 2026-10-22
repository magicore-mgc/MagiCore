# Bun/Deno lockfile reader

English: This module preserves the public `scanners::bun_deno::*` API while separating npm pin data, Bun JSONC parsing, Deno JSON parsing, package-spec splitting, and project lock aggregation. `NpmPin` deliberately remains declared in the historical facade because its GitNexus impact is CRITICAL and unresolved; its external path is unchanged.

Tiếng Việt: Module này giữ API công khai `scanners::bun_deno::*` và tách dữ liệu npm pin, parser JSONC Bun, parser JSON Deno, bộ tách package spec và điều phối gộp lock dự án. `NpmPin` được giữ ở facade cũ vì GitNexus báo CRITICAL chưa giải quyết; đường dẫn public không đổi.

`read_bun_lock` reads JSONC and extracts the first array string as a `name@version` pin. Non-pin identifiers are explicitly recorded as skipped. `read_deno_lock` reads npm keys and records JSR keys as skipped. JSONC cleanup remains shared through `mgc-types::strip_jsonc`.

`read_js_lockfiles` reads every present `bun.lock` and `deno.lock`, appending the results in Bun-then-Deno order. It does not read `mgc.lock`. The repository source map found no production caller for this helper; it remains exported and no claim is made that the audit engine invokes it.

Nearest-owner tests cover parser formats, stable error strings, scoped name splitting, and reading both project lockfiles. The existing integration suite remains unchanged. All new tests are authored but NOT_RUN; compiler, formatter, CI, and OS execution are also NOT_RUN in the source-first phase.
