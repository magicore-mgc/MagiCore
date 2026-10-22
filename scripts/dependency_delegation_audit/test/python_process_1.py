"""PythonProcessInventory test slice 1; method bodies retained verbatim."""

from .support import *

class PythonProcessInventoryPart1(unittest.TestCase):
    def test_portable_ast_dump_omits_only_empty_structural_fields(self):
        call = ast.parse('subprocess.run(["tool", "keywords=[]", "None"])').body[0].value
        self.assertEqual(
            gate._portable_ast_dump(call),
            "Call(func=Attribute(value=Name(id='subprocess', ctx=Load()), attr='run', ctx=Load()), args=[List(elts=[Constant(value='tool'), Constant(value='keywords=[]'), Constant(value='None')], ctx=Load())])",
        )
        changed = ast.parse('subprocess.run(["tool", "keywords=[]", "None"], shell=True)').body[0].value
        self.assertNotEqual(gate._portable_ast_dump(call), gate._portable_ast_dump(changed))

    def test_python_process_helper_review_binds_the_selected_executable(self):
        source = (
            "def select_shell():\n"
            "    return 'bash'\n"
        )
        function = ast.parse(source).body[0]
        fingerprint = gate._python_process_helper_fingerprint(function)
        path = "scripts/example.py"
        reviews = {
            path: {
                "select_shell": (
                    fingerprint,
                    "Selects the intended shell executable for the platform.",
                ),
            },
        }

        with mock.patch.dict(gate.PYTHON_PROCESS_HELPER_REVIEWS, reviews, clear=True):
            approved = gate.review_python_process_helpers(
                {path: source}, expected_files={path}
            )
            changed = gate.review_python_process_helpers(
                {path: source.replace("'bash'", "'sh'")}, expected_files={path}
            )
            rebound = gate.review_python_process_helpers(
                {path: source + "select_shell = lambda: 'attacker.exe'\n"},
                expected_files={path},
            )
            imported = gate.review_python_process_helpers(
                {path: source + "from attacker import select_shell\n"},
                expected_files={path},
            )
            wildcard = gate.review_python_process_helpers(
                {path: source + "from attacker import *\n"},
                expected_files={path},
            )
            namespace_write = gate.review_python_process_helpers(
                {path: source + "globals()['select_shell'] = lambda: 'attacker.exe'\n"},
                expected_files={path},
            )
            function_globals_write = gate.review_python_process_helpers(
                {
                    path: source
                    + "select_shell.__globals__['select_shell'] = "
                    + "lambda: 'attacker.exe'\n"
                },
                expected_files={path},
            )
            builtin_alias = gate.review_python_process_helpers(
                {
                    path: source
                    + "from builtins import globals as module_namespace\n"
                    + "module_namespace()['select_shell'] = lambda: 'attacker.exe'\n"
                },
                expected_files={path},
            )
            module_attribute = gate.review_python_process_helpers(
                {
                    path: source
                    + "import sys\n"
                    + "sys.modules[__name__].select_shell = lambda: 'attacker.exe'\n"
                },
                expected_files={path},
            )
            removed = gate.review_python_process_helpers({}, expected_files={path})

        self.assertEqual([item["review_status"] for item in approved], ["reviewed"])
        self.assertEqual([item["review_status"] for item in changed], ["stale-review"])
        self.assertEqual([item["review_status"] for item in rebound], ["stale-review"])
        self.assertEqual([item["review_status"] for item in imported], ["stale-review"])
        self.assertEqual([item["review_status"] for item in wildcard], ["stale-review"])
        self.assertEqual(
            [item["review_status"] for item in namespace_write], ["stale-review"]
        )
        self.assertEqual(
            [item["review_status"] for item in function_globals_write],
            ["stale-review"],
        )
        self.assertEqual(
            [item["review_status"] for item in builtin_alias], ["stale-review"]
        )
        self.assertEqual(
            [item["review_status"] for item in module_attribute], ["stale-review"]
        )
        self.assertEqual([item["review_status"] for item in removed], ["stale-review"])

    def test_python_helper_fingerprint_is_stable_across_empty_ast_type_params(self):
        source = "def select_shell():\n    return 'bash'\n"
        without_type_params = ast.parse(source).body[0]
        with_type_params = ast.parse(source).body[0]
        with_type_params.type_params = []

        self.assertEqual(
            gate._python_process_helper_fingerprint(without_type_params),
            gate._python_process_helper_fingerprint(with_type_params),
        )

        with_type_params.type_params = [ast.Name(id="Shell", ctx=ast.Load())]
        self.assertNotEqual(
            gate._python_process_helper_fingerprint(without_type_params),
            gate._python_process_helper_fingerprint(with_type_params),
        )


__all__ = ['PythonProcessInventoryPart1']
