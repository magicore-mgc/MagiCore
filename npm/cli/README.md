# @magicore/cli

The [MagiCore](https://github.com/mingd-153/MagiCore) command-line tool (`mgc`) —
scaffold, install, build, dev and test projects across the `web`, `ai`, `app`
and `lib` core families.

This npm package is a thin launcher. It has no dependencies and downloads
nothing at runtime: npm installs the prebuilt binary for your platform from a
matching optional package (`@magicore/cli-<platform>-<arch>`), and `mgc` runs it.

## Install

Global install (recommended — gives you `mgc` and `magicore`):

```sh
npm install -g @magicore/cli
mgc --version
```

One-off run without installing:

```sh
npx @magicore/cli --help
```

Requires Node.js 18 or newer.

## Supported platforms

| Platform | Package |
|---|---|
| Linux x64 (glibc) | `@magicore/cli-linux-x64` |
| Linux arm64 (glibc) | `@magicore/cli-linux-arm64` |
| macOS Intel | `@magicore/cli-darwin-x64` |
| macOS Apple Silicon | `@magicore/cli-darwin-arm64` |
| Windows x64 | `@magicore/cli-win32-x64` |

Alpine/musl and Windows arm64 are not published. Use the standalone installer
described in the [installation docs](https://github.com/mingd-153/MagiCore#installation)
for those, or run the Windows x64 build under Prism emulation with
`npm install --force --cpu=x64 --os=win32 -g @magicore/cli`.

## Usage

```sh
mgc --help                 # every command, with examples
mgc create-web myapp       # scaffold a web project
mgc install                # resolve dependencies
mgc dev                    # start the dev server
```

`mgc` behaves the same whether it comes from npm or a standalone download:
exit codes, `--json` output and piped output are identical.

## Troubleshooting

**`mgc: the binary for <platform> is not installed`**
The platform package was skipped. Reinstall without `--omit=optional` /
`--no-optional`:

```sh
npm install -g @magicore/cli --force
```

**`mgc: <platform> is not a supported platform for this package`**
See *Supported platforms* above.

## Licence

MIT — see [LICENSE](./LICENSE) or the
[repository LICENSE](https://github.com/mingd-153/MagiCore/blob/main/LICENSE).
