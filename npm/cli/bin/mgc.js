#!/usr/bin/env node
// MagiCore `mgc` launcher.
//
// The real binary ships in a per-platform optional dependency
// (`@magicore/cli-<platform>-<arch>`) so npm installs exactly one download.
// This shim only locates that binary and hands it the user's argv — it never
// downloads anything at runtime and never rewrites the arguments.
//
// Kept to plain CommonJS with no dependencies so startup stays in the
// tens-of-milliseconds range: a CLI run hundreds of times a day must not pay
// for a module loader, a network call, or a config parse.
// ---------------------------------------------------------------------------
// Cầu nối `mgc`: binary thật nằm trong optional dependency theo nền tảng,
// shim chỉ tìm binary rồi chuyển nguyên argv — không tải gì lúc chạy.

'use strict';

const { spawnSync } = require('node:child_process');
const path = require('node:path');
const fs = require('node:fs');

const BINARY_NAME = process.platform === 'win32' ? 'mgc.exe' : 'mgc';

// Supported platform packages, in the order they are documented.
// Gói nền tảng được hỗ trợ, theo thứ tự tài liệu.
const SUPPORTED = [
  '@magicore/cli-linux-x64',
  '@magicore/cli-linux-arm64',
  '@magicore/cli-darwin-x64',
  '@magicore/cli-darwin-arm64',
  '@magicore/cli-win32-x64',
];

function platformPackageName() {
  return `@magicore/cli-${process.platform}-${process.arch}`;
}

function isSupported(candidate) {
  return SUPPORTED.includes(candidate);
}

// Exit codes are part of the contract: 127 means "launcher could not find a
// binary" and is distinct from any mgc exit status.
// Mã thoát là hợp đồng: 127 = không tìm thấy binary, tách khỏi mã của mgc.
const EXIT_NO_BINARY = 127;

function fail(message, hints) {
  process.stderr.write(`mgc: ${message}\n`);
  for (const hint of hints) {
    process.stderr.write(`  ${hint}\n`);
  }
  process.exit(EXIT_NO_BINARY);
}

function resolveBinary() {
  const pkgName = platformPackageName();

  if (!isSupported(pkgName)) {
    fail(
      `${process.platform}-${process.arch} is not a supported platform for this package.`,
      [
        `Supported platforms: ${SUPPORTED.map((name) => name.replace('@magicore/cli-', '')).join(', ')}.`,
        'Use the standalone installer instead:',
        '  https://github.com/mingd-153/MagiCore#installation',
        'On Windows ARM64, run the x64 build through Prism emulation:',
        '  npm install --force --cpu=x64 --os=win32 -g @magicore/cli',
      ],
    );
  }

  let packageJsonPath;
  try {
    // `package.json` is always a runtime file (never stripped), so resolving
    // it locates a package that declares the binary for this platform.
    // `package.json` luôn tồn tại lúc chạy nên resolve nó là cách chắc nhất
    // để định vị gói có khai báo binary.
    packageJsonPath = require.resolve(`${pkgName}/package.json`);
  } catch {
    fail(
      `the binary for ${process.platform}-${process.arch} is not installed (${pkgName} is missing).`,
      [
        'npm normally installs it automatically. If it was skipped, reinstall without --omit=optional:',
        `  npm install -g ${pkgName}@${require('../package.json').version}`,
        'If you installed with --no-optional, remove that flag and reinstall @magicore/cli.',
      ],
    );
  }

  const binaryPath = path.join(path.dirname(packageJsonPath), BINARY_NAME);

  if (!fs.existsSync(binaryPath)) {
    fail(
      `${pkgName} is installed but ${BINARY_NAME} is missing from it.`,
      [
        'The package may be corrupt or only partially extracted. Reinstall it with:',
        `  npm install -g ${pkgName}@${require('../package.json').version} --force`,
      ],
    );
  }

  return binaryPath;
}

function main() {
  const binaryPath = resolveBinary();
  const result = spawnSync(binaryPath, process.argv.slice(2), { stdio: 'inherit' });

  if (result.error) {
    if (result.error.code === 'EACCES') {
      fail(`cannot execute ${binaryPath} (permission denied).`, [
        `Restore the executable bit: chmod +x '${binaryPath}'`,
      ]);
    }
    fail(`failed to start ${binaryPath}: ${result.error.message}`, [
      'Reinstall the platform package to restore a complete binary.',
    ]);
  }

  if (result.signal) {
    // Re-raise the same termination signal so shells and CI see an honest
    // exit status (npm and pipelines depend on it).
    // Dựng lại đúng signal để shell/CI nhận trạng thái trung thực.
    process.kill(process.pid, result.signal);
  }

  process.exit(result.status === null ? 1 : result.status);
}

main();
