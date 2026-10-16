# MagiCore Security Notes

**Applies to:** MagiCore 1.1.0-rc.10 source candidate

**Reviewed:** 2026-10-08

MagiCore is a project and dependency workflow tool. Its execution checks are guardrails around commands that MagiCore starts; they are not an operating-system sandbox. A child process generally runs with the current user's filesystem, network, and account permissions.

Security behavior varies by command, core, ecosystem, and supported lane. This pre-release document describes the current implementation at a high level. It does not certify every framework, language, or operating system.

## Native dependency ownership

In native mode, MagiCore is intended to own dependency resolution and installation for supported lanes. Use MagiCore commands such as `mgc install`, `mgc add`, `mgc remove`, and `mgc update` for those workflows.

The executor rejects a set of known package-manager executables in its execution scopes and checks recognized dependency-resolution or mutation invocations on protected launch paths. This is not complete semantic analysis: a renamed tool, an unrecognized wrapper, or a launch path outside the executor can evade these checks. Do not treat the checks as isolation from arbitrary project code.

### Explicit JavaScript runner compatibility

Selected web commands accept `--compat-runner npm|pnpm|yarn|bun`. This is an explicit delegation mode for the named runner and its process subtree. MagiCore prints a warning and records executions where the command supplies an audit-log path. The external runner may perform dependency or lifecycle work outside MagiCore's native resolver and lock handling. Use this mode only when that trade-off is acceptable.

The compatibility runner option is currently qualified only for the web core and the four names above. Other cores and package managers are rejected until their behavior has a separate compatibility contract. Without the option, native-mode checks remain in effect.

One exact compatibility probe, `npm --version`, is answered by MagiCore as `0.0.0` rather than launching npm. This is a shim response for tools that probe npm; it is not the installed npm version and does not establish that npm is available.

## Command execution and audit records

MagiCore uses an executable allowlist and execution scopes for commands it launches. Recognized package-manager names and dependency mutation forms are denied in native execution paths. Project test, build, and dev commands still execute project code with the user's operating-system permissions; allowlisting a runtime or compiler does not make the code safe.

Some commands append execution records to a project-local `.magicore/exec.log`. Recorded arguments are sanitized, but the log is ordinary project data: it is not tamper-proof, and commands that do not provide a log path may not write there. A log is evidence of observed executions, not proof that every child process was observed.

For commands whose scripts MagiCore parses, only supported grammar and command forms are accepted. MagiCore does not promise to interpret every package-manager script syntax; behavior depends on the command and lane.

## Known limitations

- MagiCore does not currently provide general OS-level sandboxing for project processes. Launched code may access resources available to the user.
- Process-name and invocation checks are policy guardrails, not a security boundary against a deliberately evasive program. Renaming executables, adding wrappers, or starting processes outside covered launch paths may bypass them.
- Static dependency-delegation auditing has known coverage gaps for dynamic process creation, indirect wrappers, shell evaluation, and workflow-provisioned tools. A clean audit result is not proof that all delegation paths were examined.
- Compatibility mode deliberately expands what the named external package manager can do. Native dependency ownership claims do not apply to delegated work in that subtree.
- `mgc audit` coverage depends on the ecosystem lane and advisory data source available to that scan. A clean or partial report does not prove that a project has no vulnerabilities.
- Support is lane-specific. Consult `mgc capabilities`, the release workflow evidence, and the README support matrix before relying on a framework or operation.

## Reporting a vulnerability

Use the repository's **Security → Report a vulnerability** flow to send a private report. Private vulnerability reporting is enabled for `mingd-153/MagiCore` as of 2026-10-08. Do not post unpatched vulnerability details in a public issue. Include the affected MagiCore version, affected command/core, reproduction steps, and impact; avoid sending credentials or unrelated private data.

## Maintainer release checks

`mgc self-update` verifies signed release manifests only when a public Ed25519 trust root is configured with `--trust-root` or `MGC_RELEASE_TRUST_ROOTS`; it does not trust a key downloaded alongside the manifest. The release workflow keeps the private signing seed in `MGC_RELEASE_SIGNING_KEY` and uses the public `MGC_RELEASE_TRUST_ROOTS` variable for verification. Never put the signing seed in the verifier workflow or distribute it with release assets.

Before publishing a stable release, maintainers should:

1. Keep GitHub private vulnerability reporting enabled and monitor incoming reports.
2. Publish support claims only for lanes with current release evidence.
3. Review the dependency-delegation audit's known coverage gaps and any exceptions.
4. Keep this document aligned with the shipped binary and its tested behavior.
