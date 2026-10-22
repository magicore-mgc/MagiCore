"""Python implementation.

Each function keeps its source behavior and reviewed fail-closed rules.
"""

from __future__ import annotations

PYTHON_SCAN_ROOTS = ["."]

PYTHON_PROCESS_FUNCTIONS = {
    "subprocess": {
        "call", "check_call", "check_output", "getoutput", "getstatusoutput",
        "Popen", "run",
    },
    "os": {"fork", "popen", "posix_spawn", "posix_spawnp", "startfile", "system"},
    "pty": {"spawn"},
    "asyncio": {"create_subprocess_exec", "create_subprocess_shell"},
}

# Exact AST fingerprints for Python process calls reviewed as repository
# tooling. A changed call expression, extra duplicate, or missing reviewed
# call invalidates its entry and returns the route to review-required.
# Fingerprints bind the review to the argv construction and subprocess options,
# while the reason records why that exact process boundary is expected.
PYTHON_PROCESS_REVIEWS = {'tests/test_scaffold_acceptance_runner.py': {('subprocess.run', 'bfb5000bd69ce3452a1825e37bbc0cacd76cb7cbc7ff6c1139036d68f8848936'): (1,
                                                                                                                                       'Runs '
                                                                                                                                       'only '
                                                                                                                                       'the '
                                                                                                                                       'fixed '
                                                                                                                                       'repository '
                                                                                                                                       'scaffold '
                                                                                                                                       'acceptance '
                                                                                                                                       'Bash '
                                                                                                                                       'script '
                                                                                                                                       'with '
                                                                                                                                       'a '
                                                                                                                                       'generated '
                                                                                                                                       'local '
                                                                                                                                       'fixture '
                                                                                                                                       'binary, '
                                                                                                                                       'isolated '
                                                                                                                                       'temporary '
                                                                                                                                       'HOME/data/cache, '
                                                                                                                                       'captured '
                                                                                                                                       'output, '
                                                                                                                                       'a '
                                                                                                                                       '20-second '
                                                                                                                                       'timeout, '
                                                                                                                                       'and '
                                                                                                                                       'startup-file '
                                                                                                                                       'environment '
                                                                                                                                       'removed; '
                                                                                                                                       'verifies '
                                                                                                                                       'runner '
                                                                                                                                       'verdicts, '
                                                                                                                                       'not '
                                                                                                                                       'product '
                                                                                                                                       'support.')},
 'docs/specs/productReadinessAudit20261012/build-cache.py': {('subprocess.run', '8751fdaf374dc887f5b90b093179398b5e2e99cc5faf1581ba92450cbb033d3e'): (1,
                                                                                                                                                      'Local '
                                                                                                                                                      'audit '
                                                                                                                                                      'E2E '
                                                                                                                                                      'helper; '
                                                                                                                                                      'calls '
                                                                                                                                                      'only '
                                                                                                                                                      'the '
                                                                                                                                                      "checkout's "
                                                                                                                                                      'mgc '
                                                                                                                                                      'binary '
                                                                                                                                                      'with '
                                                                                                                                                      'source-defined '
                                                                                                                                                      'argv, '
                                                                                                                                                      'isolated '
                                                                                                                                                      'temporary '
                                                                                                                                                      'cwd/HOME, '
                                                                                                                                                      'captured '
                                                                                                                                                      'output, '
                                                                                                                                                      'and '
                                                                                                                                                      'a '
                                                                                                                                                      'timeout.')},
 'docs/specs/productReadinessAudit20261012/probes.py': {('subprocess.run', 'e69c692857603200253248ee1653689d02fa04698850b7e932a6b04f071b8422'): (1,
                                                                                                                                                 'Local '
                                                                                                                                                 'audit '
                                                                                                                                                 'E2E '
                                                                                                                                                 'helper; '
                                                                                                                                                 'calls '
                                                                                                                                                 'only '
                                                                                                                                                 'the '
                                                                                                                                                 "checkout's "
                                                                                                                                                 'mgc '
                                                                                                                                                 'binary '
                                                                                                                                                 'with '
                                                                                                                                                 'source-defined '
                                                                                                                                                 'argv, '
                                                                                                                                                 'isolated '
                                                                                                                                                 'temporary '
                                                                                                                                                 'cwd/HOME, '
                                                                                                                                                 'captured '
                                                                                                                                                 'output, '
                                                                                                                                                 'and '
                                                                                                                                                 'a '
                                                                                                                                                 'timeout.')},
 'benchmark/scripts/bench_v2.py': {('subprocess.run', 'eb078c219148886121d02e62b5582b723ac676d5b21cf0773def7b8b10f6e952'): (1,
                                                                                                                            'Local '
                                                                                                                            'benchmark '
                                                                                                                            'helper; '
                                                                                                                            'callers '
                                                                                                                            'pass '
                                                                                                                            'fixed '
                                                                                                                            'benchmark/system-info '
                                                                                                                            'argv '
                                                                                                                            'lists.'),
                                   ('subprocess.run', '9825f868be2838f0ee5d54a7dad9376510864232f086fb81125c69fe6b7f9316'): (1,
                                                                                                                            'Runs '
                                                                                                                            'install-only '
                                                                                                                            'benchmark '
                                                                                                                            'argv '
                                                                                                                            'from '
                                                                                                                            'the '
                                                                                                                            'fixed '
                                                                                                                            'PMS '
                                                                                                                            'table '
                                                                                                                            'in '
                                                                                                                            'an '
                                                                                                                            'isolated '
                                                                                                                            'fixture '
                                                                                                                            'directory.')},
 'scripts/audit_capability_matrix.py': {('subprocess.run', '4e9ac2d91afa104d26afe1fe48e3aa3ee6bf165b4b1d3f44c824250b25fcd44d'): (1,
                                                                                                                                 'Reads '
                                                                                                                                 'the '
                                                                                                                                 'current '
                                                                                                                                 'repository '
                                                                                                                                 'HEAD '
                                                                                                                                 'using '
                                                                                                                                 'fixed '
                                                                                                                                 'git '
                                                                                                                                 'arguments.'),
                                        ('subprocess.run', '3afc406429a0d8dcfdee7a38f25a013b2781e53df69360466b405a644d560c45'): (1,
                                                                                                                                 'Reads '
                                                                                                                                 'repository '
                                                                                                                                 'dirty '
                                                                                                                                 'state '
                                                                                                                                 'using '
                                                                                                                                 'fixed '
                                                                                                                                 'git '
                                                                                                                                 'arguments.'),
                                        ('subprocess.run', 'a11d00e9c720f21b59dded08fa29501260f450b2a90717cad7a65fc3a167e8a2'): (1,
                                                                                                                                 'Runs '
                                                                                                                                 'only '
                                                                                                                                 'the '
                                                                                                                                 'two '
                                                                                                                                 'fixed '
                                                                                                                                 'audit '
                                                                                                                                 'E2E '
                                                                                                                                 'cargo '
                                                                                                                                 'test '
                                                                                                                                 'suites '
                                                                                                                                 'with '
                                                                                                                                 'a '
                                                                                                                                 'configured '
                                                                                                                                 'timeout.')},
 'scripts/provenance_chain.py': {('subprocess.run', 'da765a20acc92310eb3d7ea67493e8d4e06607601d66e345801501bb18ce591c'): (1,
                                                                                                                          'Git '
                                                                                                                          'metadata '
                                                                                                                          'helper; '
                                                                                                                          'all '
                                                                                                                          'current '
                                                                                                                          'call '
                                                                                                                          'sites '
                                                                                                                          'use '
                                                                                                                          'fixed '
                                                                                                                          'read-only '
                                                                                                                          'git '
                                                                                                                          'subcommands.'),
                                 ('subprocess.run', '7ca4d382a4b5701bc9ea0a0de05ce02e8f970dc013999128273d281d6ee295b5'): (1,
                                                                                                                          'Checks '
                                                                                                                          'working-tree '
                                                                                                                          'state '
                                                                                                                          'using '
                                                                                                                          'fixed '
                                                                                                                          'read-only '
                                                                                                                          'git '
                                                                                                                          'arguments.')},
 'scripts/test_release_script.py': {('subprocess.run', '30189088871d987f63f90637069f395153c2256ba8d9fe638e82440f4b03ea11'): (1,
                                                                                                                             'Runs '
                                                                                                                             'the '
                                                                                                                             'local '
                                                                                                                             'release.sh '
                                                                                                                             'against '
                                                                                                                             'a '
                                                                                                                             'temporary '
                                                                                                                             'fake '
                                                                                                                             'cargo '
                                                                                                                             'executable; '
                                                                                                                             'it '
                                                                                                                             'validates '
                                                                                                                             'argv '
                                                                                                                             'and '
                                                                                                                             'never '
                                                                                                                             'builds '
                                                                                                                             'or '
                                                                                                                             'publishes '
                                                                                                                             'artifacts.')},
 'scripts/test_audit_capability_matrix.py': {('subprocess.run', 'd8794996e3739e075be94219477e820eb1884f5178a30c5050c2dac04c6a60aa'): (1,
                                                                                                                                      'Creates '
                                                                                                                                      'an '
                                                                                                                                      'isolated '
                                                                                                                                      'temporary '
                                                                                                                                      'Git '
                                                                                                                                      'repository '
                                                                                                                                      'for '
                                                                                                                                      'a '
                                                                                                                                      'unit '
                                                                                                                                      'test.'),
                                             ('subprocess.run', 'c8dd6f5373c83c8885fbed9206080e077999e4c5310624c6697738c20e9849e1'): (1,
                                                                                                                                      'Sets '
                                                                                                                                      'a '
                                                                                                                                      'test-only '
                                                                                                                                      'email '
                                                                                                                                      'identity '
                                                                                                                                      'in '
                                                                                                                                      'the '
                                                                                                                                      'isolated '
                                                                                                                                      'temporary '
                                                                                                                                      'Git '
                                                                                                                                      'repository.'),
                                             ('subprocess.run', '3cd39da9c1c3b6b5abba781e59ee53375341bf1a75536f8afe0294bfd97c7acd'): (1,
                                                                                                                                      'Sets '
                                                                                                                                      'a '
                                                                                                                                      'test-only '
                                                                                                                                      'name '
                                                                                                                                      'identity '
                                                                                                                                      'in '
                                                                                                                                      'the '
                                                                                                                                      'isolated '
                                                                                                                                      'temporary '
                                                                                                                                      'Git '
                                                                                                                                      'repository.'),
                                             ('subprocess.run', 'fcc0b22c8eef8e5747a25d94d3528f56395765d2f50ac443fdc4346e286d393f'): (1,
                                                                                                                                      'Stages '
                                                                                                                                      'only '
                                                                                                                                      'the '
                                                                                                                                      'test '
                                                                                                                                      'fixture '
                                                                                                                                      'file '
                                                                                                                                      'in '
                                                                                                                                      'the '
                                                                                                                                      'isolated '
                                                                                                                                      'temporary '
                                                                                                                                      'Git '
                                                                                                                                      'repository.'),
                                             ('subprocess.run', '177895e8d7981a0613e09f616fcda3eb6df22224ba0f4de3163d0d378422389e'): (1,
                                                                                                                                      'Creates '
                                                                                                                                      'the '
                                                                                                                                      'base '
                                                                                                                                      'commit '
                                                                                                                                      'in '
                                                                                                                                      'the '
                                                                                                                                      'isolated '
                                                                                                                                      'temporary '
                                                                                                                                      'Git '
                                                                                                                                      'repository.')},
 'scripts/lifecycle_matrix/validation/test/test_flutter_sdk_bootstrap.py': {('subprocess.run', '4ae5f5630c8367a9a3d9c0823cb0fac5066b407f7cbe46d09222fea3d3574497'): (1,
                                                                                                                                                                     'Runs '
                                                                                                                                                                     'the '
                                                                                                                                                                     'fixed '
                                                                                                                                                                     'Flutter '
                                                                                                                                                                     'bootstrap '
                                                                                                                                                                     'script '
                                                                                                                                                                     'with '
                                                                                                                                                                     'POSIX '
                                                                                                                                                                     'Bash '
                                                                                                                                                                     'or '
                                                                                                                                                                     'the '
                                                                                                                                                                     'selected '
                                                                                                                                                                     'Git '
                                                                                                                                                                     'Bash '
                                                                                                                                                                     'executable '
                                                                                                                                                                     'and '
                                                                                                                                                                     'fake '
                                                                                                                                                                     'dart/flutter '
                                                                                                                                                                     'executables '
                                                                                                                                                                     'in '
                                                                                                                                                                     'a '
                                                                                                                                                                     'temporary '
                                                                                                                                                                     'SDK '
                                                                                                                                                                     'tree.'),
                                                                            ('subprocess.run', '25c931822c0d7274b11af4b171494fe34324d14d613957c93024c6a36662fe40'): (1,
                                                                                                                                                                     'Runs '
                                                                                                                                                                     'a '
                                                                                                                                                                     'fixed '
                                                                                                                                                                     'Bash '
                                                                                                                                                                     'command '
                                                                                                                                                                     'with '
                                                                                                                                                                     'the '
                                                                                                                                                                     'platform-selected '
                                                                                                                                                                     'shell '
                                                                                                                                                                     'to '
                                                                                                                                                                     'source '
                                                                                                                                                                     'the '
                                                                                                                                                                     'repository '
                                                                                                                                                                     'bootstrap '
                                                                                                                                                                     'and '
                                                                                                                                                                     'resolve '
                                                                                                                                                                     'a '
                                                                                                                                                                     'constant '
                                                                                                                                                                     'Windows '
                                                                                                                                                                     'fixture '
                                                                                                                                                                     'path.'),
                                                                            ('subprocess.run', '9172501f4f16afe766e79a03913395aef0c450371f411c00724990dc12b5126f'): (1,
                                                                                                                                                                     'Compiles '
                                                                                                                                                                     'a '
                                                                                                                                                                     'native '
                                                                                                                                                                     'test-only '
                                                                                                                                                                     'dart.exe '
                                                                                                                                                                     'stub '
                                                                                                                                                                     'with '
                                                                                                                                                                     'fixed '
                                                                                                                                                                     'rustc '
                                                                                                                                                                     'arguments '
                                                                                                                                                                     'for '
                                                                                                                                                                     'the '
                                                                                                                                                                     'Windows '
                                                                                                                                                                     'Flutter '
                                                                                                                                                                     'bootstrap '
                                                                                                                                                                     'fixture; '
                                                                                                                                                                     'no '
                                                                                                                                                                     'package '
                                                                                                                                                                     'or '
                                                                                                                                                                     'network '
                                                                                                                                                                     'mutation.')},
 'scripts/test_github_action_pin.py': {('subprocess.run', 'b47a6572ab2b6f53fef5ad59a30ea2d07e32efd59aee7c826620234a1e126f21'): (1,
                                                                                                                                'Runs '
                                                                                                                                'a '
                                                                                                                                'test-only '
                                                                                                                                'bash '
                                                                                                                                'verifier '
                                                                                                                                'with '
                                                                                                                                'positional '
                                                                                                                                'arguments '
                                                                                                                                'and '
                                                                                                                                'a '
                                                                                                                                'fake '
                                                                                                                                'git '
                                                                                                                                'on '
                                                                                                                                'the '
                                                                                                                                'test '
                                                                                                                                'PATH.')},
 'scripts/test_inventory.py': {('subprocess.run', '8dceba98656b0f0b839e2fd1b3b0788e5f94ad93029626a1bc999942cbc4bf7c'): (2,
                                                                                                                        'Reads '
                                                                                                                        'HEAD '
                                                                                                                        'for '
                                                                                                                        'local '
                                                                                                                        'test '
                                                                                                                        'inventory '
                                                                                                                        'metadata '
                                                                                                                        'using '
                                                                                                                        'fixed '
                                                                                                                        'git '
                                                                                                                        'arguments.'),
                               ('subprocess.run', 'c3630e647f65713d17db619174b4411964beeecbf7e0eb2c85997a63986fc725'): (1,
                                                                                                                        'Runs '
                                                                                                                        'the '
                                                                                                                        'fixed '
                                                                                                                        'default '
                                                                                                                        'cargo '
                                                                                                                        'test '
                                                                                                                        'inventory '
                                                                                                                        'command '
                                                                                                                        'and '
                                                                                                                        'captures '
                                                                                                                        'test '
                                                                                                                        'results.')},
 'tools/mgc-mcp/mcp_server.py': {('subprocess.run', 'bdbb0096bdaa5a5ad909cc82acc2bbb7a1d7dada88c40ad6715240d8f15802e0'): (1,
                                                                                                                          'MCP '
                                                                                                                          'dispatch '
                                                                                                                          'passes '
                                                                                                                          'argv '
                                                                                                                          'without '
                                                                                                                          'a '
                                                                                                                          'shell; '
                                                                                                                          'subcommands '
                                                                                                                          'are '
                                                                                                                          'selected '
                                                                                                                          'from '
                                                                                                                          'ARGS_FOR_TOOL '
                                                                                                                          'and '
                                                                                                                          'timeout '
                                                                                                                          'is '
                                                                                                                          'fixed.')},
 'scripts/dependency_delegation_audit/runner/runner.py': {('subprocess.run', '09480a7bb71a9ff8fa5c6c32b1b09f4c7949d20fdf4bcebaaaf48ef535545a23'): (1,
                                                                                                                                                   'Reads '
                                                                                                                                                   'repository '
                                                                                                                                                   'HEAD '
                                                                                                                                                   'for '
                                                                                                                                                   'the '
                                                                                                                                                   'generated '
                                                                                                                                                   'audit '
                                                                                                                                                   'ledger '
                                                                                                                                                   'using '
                                                                                                                                                   'fixed '
                                                                                                                                                   'git '
                                                                                                                                                   'arguments.'),
                                                          ('subprocess.run', '5e23c44f5fcf447106ad3d48651774d3cafdb40e679347ce60079aa9348995c3'): (1,
                                                                                                                                                   'Reads '
                                                                                                                                                   'repository '
                                                                                                                                                   'dirty '
                                                                                                                                                   'state '
                                                                                                                                                   'for '
                                                                                                                                                   'the '
                                                                                                                                                   'generated '
                                                                                                                                                   'audit '
                                                                                                                                                   'ledger '
                                                                                                                                                   'using '
                                                                                                                                                   'fixed '
                                                                                                                                                   'git '
                                                                                                                                                   'arguments.')},
 'scripts/lifecycle_matrix/process/process.py': {('subprocess.run', '7cfeb3cb2645367ecc7f91a954cb0666b366f378a78a572e6fcd9bda09c2f298'): (1,
                                                                                                                                          'Shared '
                                                                                                                                          'lifecycle '
                                                                                                                                          'evidence '
                                                                                                                                          'runner; '
                                                                                                                                          'call '
                                                                                                                                          'sites '
                                                                                                                                          'execute '
                                                                                                                                          'bounded, '
                                                                                                                                          'source-defined '
                                                                                                                                          'test/build '
                                                                                                                                          'commands.')},
 'scripts/lifecycle_matrix/runner/dev_probe.py': {('subprocess.Popen', 'a53a245f97a247c95efef054c891d2ed388e947173ba179567a90f518a662f98'): (1,
                                                                                                                                             'Starts '
                                                                                                                                             'the '
                                                                                                                                             'local '
                                                                                                                                             'mgc '
                                                                                                                                             'dev '
                                                                                                                                             'server '
                                                                                                                                             'for '
                                                                                                                                             'a '
                                                                                                                                             'bounded '
                                                                                                                                             'lifecycle '
                                                                                                                                             'readiness '
                                                                                                                                             'probe.')},
 'scripts/lifecycle_matrix/runner/recovery.py': {('subprocess.Popen', '7b86bfefe6e49f3ae4cbd67739e790506858031072fc183fba63766f68b48bf8'): (1,
                                                                                                                                            'Starts '
                                                                                                                                            'the '
                                                                                                                                            'local '
                                                                                                                                            'mgc '
                                                                                                                                            'binary '
                                                                                                                                            'for '
                                                                                                                                            'a '
                                                                                                                                            'recovery '
                                                                                                                                            'fault-injection '
                                                                                                                                            'fixture '
                                                                                                                                            'and '
                                                                                                                                            'captures '
                                                                                                                                            'its '
                                                                                                                                            'output.')},
 'scripts/lifecycle_matrix/runner/run_probe.py': {('subprocess.Popen', 'ef9a39ec0d317a4020fd6b5ed8a52b9aa8d36c7a076e3ac1caad9425466e5133'): (1,
                                                                                                                                             'Starts '
                                                                                                                                             'the '
                                                                                                                                             'local '
                                                                                                                                             'mgc '
                                                                                                                                             'binary '
                                                                                                                                             'with '
                                                                                                                                             'a '
                                                                                                                                             'source-defined '
                                                                                                                                             'lifecycle '
                                                                                                                                             'probe '
                                                                                                                                             'argv '
                                                                                                                                             'in '
                                                                                                                                             'a '
                                                                                                                                             'sandbox '
                                                                                                                                             'project.')},
 'stress-test/src/stress_matrix_engine/evidence/source_provenance/source_provenance.py': {('subprocess.run', '25d3e990bd8612601ad5b36cd197efae064275399bbbe331addbced29003c4f9'): (1,
                                                                                                                                                                                   'Reads '
                                                                                                                                                                                   'the '
                                                                                                                                                                                   'repository '
                                                                                                                                                                                   'HEAD '
                                                                                                                                                                                   'with '
                                                                                                                                                                                   'fixed '
                                                                                                                                                                                   'read-only '
                                                                                                                                                                                   '`git '
                                                                                                                                                                                   'rev-parse '
                                                                                                                                                                                   'HEAD` '
                                                                                                                                                                                   'arguments, '
                                                                                                                                                                                   'repository '
                                                                                                                                                                                   'cwd, '
                                                                                                                                                                                   'captured '
                                                                                                                                                                                   'output, '
                                                                                                                                                                                   'and '
                                                                                                                                                                                   'timeout.'),
                                                                                          ('subprocess.run', 'f0dd10cfdb69fc53b1ebec027f6c7b3d2a35275a28f762d3392bdbae21ce79e5'): (1,
                                                                                                                                                                                   'Reads '
                                                                                                                                                                                   'repository '
                                                                                                                                                                                   'dirty '
                                                                                                                                                                                   'state '
                                                                                                                                                                                   'with '
                                                                                                                                                                                   'fixed '
                                                                                                                                                                                   'read-only '
                                                                                                                                                                                   '`git '
                                                                                                                                                                                   'status '
                                                                                                                                                                                   '--porcelain=v1` '
                                                                                                                                                                                   'arguments, '
                                                                                                                                                                                   'repository '
                                                                                                                                                                                   'cwd, '
                                                                                                                                                                                   'captured '
                                                                                                                                                                                   'output, '
                                                                                                                                                                                   'and '
                                                                                                                                                                                   'timeout.')},
 'stress-test/src/stress_matrix_engine/process/run_command/run_command.py': {('subprocess.run', '2a29fe6a0d0063d10a3aeac3db2ef1a138c63cc145b81d15578ebbbbff4bb529'): (1,
                                                                                                                                                                      'Runs '
                                                                                                                                                                      'only '
                                                                                                                                                                      'the '
                                                                                                                                                                      'selected '
                                                                                                                                                                      'local '
                                                                                                                                                                      'MagiCore '
                                                                                                                                                                      'binary '
                                                                                                                                                                      'with '
                                                                                                                                                                      'a '
                                                                                                                                                                      'source-defined '
                                                                                                                                                                      'argv '
                                                                                                                                                                      'vector, '
                                                                                                                                                                      'isolated '
                                                                                                                                                                      'project '
                                                                                                                                                                      'cwd/environment, '
                                                                                                                                                                      'captured '
                                                                                                                                                                      'output, '
                                                                                                                                                                      'timeout, '
                                                                                                                                                                      'and '
                                                                                                                                                                      'no '
                                                                                                                                                                      'shell.')}}

# Source-bound reviews for helpers that select the executable passed through a
# separately fingerprinted subprocess call. This closes the gap where a call
# still looks identical while the variable feeding its argv changes behavior.
# (Review source-bound cho hàm chọn executable đưa vào subprocess; ngăn việc
# giữ nguyên biểu thức call nhưng đổi hành vi của biến argv.)
PYTHON_PROCESS_HELPER_REVIEWS = {
    "scripts/lifecycle_matrix/validation/test/support.py": {
        "bootstrap_bash_command": (
            "fdf806be04167c38b93b21ab0db79ebc01cac22a5f0c5aacfbc87cfc09ef1032",
            "Selects POSIX Bash or resolves Git Bash beside Git for Windows for the fixed Flutter bootstrap fixture.",
        ),
    },
}

PYTHON_PROCESS_HELPER_DEPENDENCIES = {
    "scripts/lifecycle_matrix/validation/test/support.py": {
        "bootstrap_bash_command": {
            "os": "os",
            "shutil": "shutil",
            "Path": "pathlib.Path",
        },
    },
}

__all__ = []
