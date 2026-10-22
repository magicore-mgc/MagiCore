<div align="center">
  <img src="assets/logo-full.svg" alt="MagiCore" width="full" />
  <h1>MagiCore</h1>
  <p><strong>A Rust CLI for project and dependency workflows across multiple cores</strong></p>
  <p><a href="https://github.com/mingd-153/MagiCore/actions/workflows/ci.yml">CI</a> · <a href="https://github.com/mingd-153/MagiCore/pulls">Pull requests</a> · <a href="LICENSE">MIT license</a></p>
</div>

---

MagiCore (<code>mgc</code>) is a Rust CLI and workspace for scaffolding projects and coordinating dependency, test, build, audit, and release workflows. The CLI exposes nine core families. **Support and maturity differ by core, framework, and operation.**

## Current status

- Workspace version: **1.1.0-rc.10**; Rust edition 2024; minimum Rust version 1.94.
- This is a pre-release, not a production-stable or drop-in replacement claim.
- CI evidence is commit-specific. Before merging or tagging, review the PR's Checks and workflow artifacts for the exact source SHA; runs on another SHA do not qualify this checkout.
- Green CI applies to declared lanes; it does not qualify every framework or operation.

## What MagiCore provides

- Core-aware project scaffolding and capability reporting.
- Dependency workflows: install, add, remove, update, and list.
- Project test, build, run, verify, and audit commands where supported.
- Lockfile import, SBOM generation, registry publishing, workspace inspection, and cache management.
- A native Model Context Protocol server: <code>mgc mcp</code>.

Use <code>mgc capabilities</code> to inspect the capability manifest and <code>mgc &lt;command&gt; --help</code> for options in this build.

### Project packages and persistent CLI tools

In the current development tree, <code>mgc add &lt;package&gt;</code> adds a dependency to the detected project. <code>mgc add --global</code> is rejected; use an explicit command for each install scope:

~~~bash
# Temporary npm executable; the temporary project is cleaned after it exits.
mgc dlx opencode-ai@1.18.35 --allow-scripts -- --version

# Persistent user-level npm CLI; use --bin when the package exposes several commands.
mgc tool install npm:@openai/codex@0.162.0 --bin codex
mgc tool list
mgc tool update codex --version latest
mgc tool remove codex
~~~

Persistent tools live in <code>$MGC_DATA_DIR/tools</code>, with command shims in <code>$MGC_DATA_DIR/bin</code>; the default data directory is <code>~/MagiCore-SystemData</code>. MagiCore adds the shim directory to the current user's shell PATH, and a new terminal is needed to pick up the change. The persistent installer currently supports explicit <code>npm:</code> sources through a build that includes the Web core. It does not claim global installation for Cargo, PyPI, Go, NuGet, or other ecosystems. <code>dlx</code>, tool install, and tool update disable lifecycle scripts unless <code>--allow-scripts</code> is passed; enabled scripts run with the current user's permissions, so only opt in for a package you trust. JavaScript CLIs also need a Node runtime available on PATH.

The release-binary macOS fixture E2Es currently pass for persistent tools (4/4) and <code>dlx</code> script policy (1/1). A live <code>opencode-ai@1.18.35</code> run with <code>--allow-scripts</code> also completed in an isolated temporary HOME and data directory and printed version <code>1.18.35</code>. A separate persistent-tool smoke test installed and updated that package, listed it, ran the managed shim, and removed it; its record stored the normalized target <code>bin/opencode.exe</code>. The lifecycle workflow is wired to run the fixture tests, plus the AI model-audit regression, on Linux, macOS, and Windows. The current dirty working tree has no hosted exact-candidate result.

## Core support and CI evidence

The latest inspected hosted results are for base SHA `8ce61f227b9d32cb1ced3db4fa1804992daac17b`. The [lifecycle capability matrix](https://github.com/mingd-153/MagiCore/actions/runs/38380589722), [native dependency lifecycle matrix](https://github.com/mingd-153/MagiCore/actions/runs/38380589784), and [release archive E2E](https://github.com/mingd-153/MagiCore/actions/runs/38380589931) passed on their configured OS runners. The [CI run](https://github.com/mingd-153/MagiCore/actions/runs/38380589750) failed on a Linux process-escape regression; the [stress run](https://github.com/mingd-153/MagiCore/actions/runs/38380589772) failed on the Windows AI model-path audit and reported 1,615 universal-support evidence/ownership gaps. The dirty working tree includes local fixes and new release-binary E2Es, but it has no exact-SHA hosted run yet. Passing scaffold or CLI contract checks do not prove complete lifecycle support for every framework.

| Core | Current evidence and limits |
|---|---|
| Web | Native JavaScript lifecycle E2E passed on macOS arm64 for 18 lanes: vanilla, TypeScript, Node, Express, Fastify, Hono, NestJS, Nuxt, Qwik, SolidJS, tRPC, Angular, Astro, Next.js, React Vite, Remix, SvelteKit, and Vue Vite. The remaining framework records and non-JavaScript web ecosystems have scaffold-only or narrower evidence; this does not qualify every web framework. |
| AI | The Python lifecycle lane passed. Agent and MCP-server scaffolds do not inherit a full lifecycle guarantee. |
| App | Flutter lifecycle passed. Swift E2E used a fallback project on all three OS because the distributed/registry scaffold was unavailable; that does not verify the Swift scaffold. Objective-C and React Native are not release-qualified. |
| Lib | Rust, Python, TypeScript, and Go lanes passed. Java and .NET remain partial or outside release scope. |
| Cloud (<code>clo</code>) | CDK and Pulumi lanes passed. Terraform dependency operations are outside the current supported scope. |
| CI/CD (<code>cicd</code>) | GitHub Actions is a workflow scaffold; install/test/build execute on the external CI provider. |
| Game | The Rust lane is partial; package lifecycle support varies by framework. |
| IoT | The Rust lane is partial; board/toolchain combinations are not all release-qualified. |
| Hardware | Optimizer and benchmark tooling; not a general package lifecycle. |

The Native Dependency Lifecycle Evidence Matrix covers selected MagiCore-native dependency-install lanes only. Check <code>mgc capabilities</code> and workflow artifacts for lane-level detail.

Native Python install can materialize compatible prebuilt wheels without invoking <code>pip</code>. For compiled wheels, set <code>MGC_PYTHON_VERSION</code> to the exact minor version used by <code>mgc run</code> or <code>mgc dev</code> (for example, <code>3.12</code>); MagiCore refuses an ABI-specific wheel when that version is unknown or incompatible. Source distributions are not built by the native installer.

## Build from source

Requirements: Rust 1.94 or newer.

~~~bash
git clone https://github.com/mingd-153/MagiCore.git
cd MagiCore
cargo build --release --bin mgc --locked
./target/release/mgc --version
./target/release/mgc capabilities
~~~

Add <code>target/release</code> to your PATH or install the binary using your platform's normal method. Tests/builds that need a language SDK still require that SDK.

## Install a published release

Release archives exist only for versions listed on the [GitHub Releases page](https://github.com/mingd-153/MagiCore/releases). The installer and current target matrix are documented in [`packaging/README.md`](packaging/README.md). The installer checks an archive against its SHA-256 sidecar; it does not verify the signed release manifest. See [`SECURITY.md`](SECURITY.md) for the trust model and limitations.

## Quick start

After making <code>mgc</code> available on PATH:

~~~bash
mgc create-web react my-app --yes
cd my-app
mgc install
mgc test
mgc build
mgc audit
~~~

Other scaffold command families include <code>create-ai</code>, <code>create-app</code>, <code>create-lib</code>, <code>create-clo</code>, <code>create-cicd</code>, <code>create-game</code>, <code>create-iot</code>, and <code>create-hardware</code>. Check capabilities before assuming a framework supports each operation.

## Storage and caches

MagiCore-managed global stores, shared caches, scaffold downloads, templates, search data, and telemetry default to <code>~/MagiCore-SystemData</code>. Set <code>MGC_DATA_DIR</code> to use another location. Existing store-specific overrides such as <code>MAGICORE_STORE_ROOT</code> remain available.

### Project-local `.magicore/`

<code>.magicore/</code> is a project-local state directory. MagiCore creates its contents when an operation needs them; it is not the core selector and an empty directory does not mean a framework is enabled or supported. The core identity is read from project manifests and, when present, the separate plain-text <code>.mgc.core</code> marker. That marker is editable text, not a signature: conflicting declarations are rejected, while a trust anchor can detect edits or a rename. Run <code>mgc trust anchor</code> and set <code>MGC_TRUST_ANCHOR=require</code> when core identity must fail closed; the default <code>warn</code> mode preserves legacy-project discovery and reports anchor problems.

Common entries have different jobs:

- <code>exec.log</code> is a JSON-lines record of executions made by commands that opt into auditing. Arguments are sanitized, but the file is ordinary editable project data and is not a complete or tamper-proof security record.
- <code>cache/web/</code> holds project-scoped Web cache data. The shared package store and other machine-wide caches live under <code>$MGC_DATA_DIR</code>.
- <code>project.id</code>, <code>journal/</code>, and <code>lock.write</code> support project identity, dependency-mutation recovery, and serialized project writes. A completed transaction clears its journal; do not remove a journal while recovery is pending.
- <code>patches/</code> contains project patch files referenced from <code>mgc.toml</code>; keep those files when the manifest uses them.
- <code>tmp-optimizer/</code> is staging space for optimizer materialization and can be recreated by that operation.

Entries are created lazily, so a project normally contains only the state its commands have used. MagiCore excludes the whole directory from generated package archives; Git ignore behavior depends on the project's own ignore rules. Use <code>mgc cache prune</code> for cache cleanup rather than deleting transaction journals or configured patches.

Run <code>mgc cache status</code> from a project to see the MagiCore data size, project size, and combined usage. The default warning threshold is 50 GiB; override it with <code>MGC_STORAGE_BUDGET_GB</code>. This is a reporting threshold, not an operating-system quota: tools launched outside MagiCore may keep writing to their own cache locations.

## Verification and performance

The repository defines six PR workflows: CI, resolver archive security, lifecycle capability evidence, release archive E2E, delegated compatibility, and the cross-platform stress matrix. Path filters mean not every workflow runs for every PR; inspect that PR's Checks and workflow artifacts for the lanes actually exercised. See the [CI workflow](.github/workflows/ci.yml), [lifecycle workflow](.github/workflows/lifecycle-matrix.yml), [release E2E workflow](.github/workflows/release-binary-e2e.yml), [delegated compatibility workflow](.github/workflows/delegated-compatibility-matrix.yml), and [stress workflow](.github/workflows/stress-matrix.yml).

This README makes no speed comparison. Do not claim an advantage over npm, pnpm, Bun, or another tool without current validated benchmarks and a published method/baseline.

## Repository layout

- <code>cli/</code> — command-line application.
- <code>core/crates/</code> — shared Rust foundations.
- <code>adapters/</code> — core-specific behavior.
- <code>tests/</code> — integration and end-to-end tests.
- <code>.github/workflows/</code> — CI and release verification.

## Contributing

Open a focused pull request with relevant unit, integration, and lifecycle evidence. Follow [AGENTS.md](AGENTS.md) and [RULE.md](RULE.md); keep public support claims aligned with CI lane verdicts.

## License

MIT. See [LICENSE](LICENSE).
