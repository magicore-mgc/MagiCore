"""PythonProcessInventory test slice 2; method bodies retained verbatim."""

from .support import *

class PythonProcessInventoryPart2(unittest.TestCase):
    def test_python_process_helper_review_binds_global_import_dependencies(self):
        source = (
            "import shutil\n"
            "from pathlib import Path\n"
            "def select_shell():\n"
            "    return shutil.which('git')\n"
        )
        function = ast.parse(source).body[2]
        fingerprint = gate._python_process_helper_fingerprint(function)
        path = "scripts/example.py"
        reviews = {
            path: {
                "select_shell": (
                    fingerprint,
                    "Selects the shell using the reviewed shutil import.",
                ),
            },
        }
        dependencies = {
            path: {
                "select_shell": {
                    "shutil": "shutil",
                    "Path": "pathlib.Path",
                },
            },
        }

        with mock.patch.dict(gate.PYTHON_PROCESS_HELPER_REVIEWS, reviews, clear=True), \
             mock.patch.dict(
                 gate.PYTHON_PROCESS_HELPER_DEPENDENCIES, dependencies, clear=True
             ):
            approved = gate.review_python_process_helpers(
                {path: source}, expected_files={path}
            )
            mutated = gate.review_python_process_helpers(
                {path: source + "shutil.which = lambda _: 'attacker.exe'\n"},
                expected_files={path},
            )
            aliased_module_mutation = gate.review_python_process_helpers(
                {
                    path: source
                    + "import shutil as alternate_shutil\n"
                    + "alternate_shutil.which = lambda _: 'attacker.exe'\n"
                },
                expected_files={path},
            )
            assigned_alias_mutation = gate.review_python_process_helpers(
                {
                    path: source
                    + "alternate_shutil = shutil\n"
                    + "alternate_shutil.which = lambda _: 'attacker.exe'\n"
                },
                expected_files={path},
            )
            class_alias_mutation = gate.review_python_process_helpers(
                {
                    path: source
                    + "from pathlib import Path as alternate_path\n"
                    + "alternate_path.resolve = lambda self: self\n"
                },
                expected_files={path},
            )
            registry_mutation_snippets = {
                "direct_index": (
                    "import sys\n"
                    "sys.modules['shutil'].which = lambda _: 'attacker.exe'\n"
                ),
                "namespace_alias": (
                    "import sys\n"
                    "registry = sys.modules\n"
                    "registry['shutil'].which = lambda _: 'attacker.exe'\n"
                ),
                "registry_copy": (
                    "import sys\n"
                    "registry = sys.modules.copy()\n"
                    "registry['shutil'].which = lambda _: 'attacker.exe'\n"
                ),
                "dict_copy": (
                    "import sys\n"
                    "registry = dict(sys.modules)\n"
                    "registry['shutil'].which = lambda _: 'attacker.exe'\n"
                ),
                "module_index_alias": (
                    "import sys\n"
                    "shutil_module = sys.modules['shutil']\n"
                    "shutil_module.which = lambda _: 'attacker.exe'\n"
                ),
                "module_get_alias": (
                    "import sys\n"
                    "shutil_module = sys.modules.get('shutil')\n"
                    "shutil_module.which = lambda _: 'attacker.exe'\n"
                ),
                "items_copy": (
                    "import sys\n"
                    "registry = dict(sys.modules.items())\n"
                    "registry['shutil'].which = lambda _: 'attacker.exe'\n"
                ),
                "dict_copy_method": (
                    "import sys\n"
                    "registry = dict.copy(sys.modules)\n"
                    "registry['shutil'].which = lambda _: 'attacker.exe'\n"
                ),
                "mapping_unpack": (
                    "import sys\n"
                    "registry = {**sys.modules}\n"
                    "registry['shutil'].which = lambda _: 'attacker.exe'\n"
                ),
                "mapping_union": (
                    "import sys\n"
                    "registry = sys.modules | {}\n"
                    "registry['shutil'].which = lambda _: 'attacker.exe'\n"
                ),
                "inline_get": (
                    "import sys\n"
                    "sys.modules.get('shutil').which = lambda _: 'attacker.exe'\n"
                ),
                "inline_copy": (
                    "import sys\n"
                    "sys.modules.copy()['shutil'].which = lambda _: 'attacker.exe'\n"
                ),
                "inline_dict": (
                    "import sys\n"
                    "dict(sys.modules)['shutil'].which = lambda _: 'attacker.exe'\n"
                ),
                "dict_comprehension": (
                    "import sys\n"
                    "registry = {key: module for key, module in sys.modules.items()}\n"
                    "registry['shutil'].which = lambda _: 'attacker.exe'\n"
                ),
                "list_values": (
                    "import sys\n"
                    "modules = list(sys.modules.values())\n"
                    "modules[0].which = lambda _: 'attacker.exe'\n"
                ),
                "tuple_values": (
                    "import sys\n"
                    "modules = tuple(sys.modules.values())\n"
                    "modules[0].which = lambda _: 'attacker.exe'\n"
                ),
                "list_comprehension": (
                    "import sys\n"
                    "modules = [module for module in sys.modules.values()]\n"
                    "modules[0].which = lambda _: 'attacker.exe'\n"
                ),
                "dynamic_import": (
                    "__import__('shutil').which = lambda _: 'attacker.exe'\n"
                ),
                "builtin_dict_alias": (
                    "import sys\n"
                    "from builtins import dict as clone_modules\n"
                    "registry = clone_modules(sys.modules)\n"
                    "registry['shutil'].which = lambda _: 'attacker.exe'\n"
                ),
                "opaque_registry_wrapper": (
                    "import sys\n"
                    "from collections import ChainMap\n"
                    "registry = ChainMap(sys.modules)\n"
                    "registry['shutil'].which = lambda _: 'attacker.exe'\n"
                ),
                "opaque_imported_registry_wrapper": (
                    "from sys import modules as registry\n"
                    "from collections import ChainMap\n"
                    "wrapped = ChainMap(registry)\n"
                    "wrapped['shutil'].which = lambda _: 'attacker.exe'\n"
                ),
                "importlib_dynamic_import": (
                    "import importlib\n"
                    "importlib.import_module('shutil').which = "
                    "lambda _: 'attacker.exe'\n"
                ),
                "importlib_loader_import": (
                    "import importlib.util\n"
                    "importlib.util.find_spec('shutil').loader.load_module().which = "
                    "lambda _: 'attacker.exe'\n"
                ),
                "aliased_importlib_find_spec": (
                    "from importlib.util import find_spec as resolve_spec\n"
                    "resolve_spec('shutil').loader.load_module().which = "
                    "lambda _: 'attacker.exe'\n"
                ),
                "importlib_module_from_spec": (
                    "import importlib.util\n"
                    "spec = importlib.util.find_spec('shutil')\n"
                    "importlib.util.module_from_spec(spec).which = "
                    "lambda _: 'attacker.exe'\n"
                ),
                "importlib_reload": (
                    "import importlib\n"
                    "importlib.reload(shutil).which = lambda _: 'attacker.exe'\n"
                ),
                "aliased_dynamic_import": (
                    "from importlib import import_module as load_module\n"
                    "load_module('shutil').which = lambda _: 'attacker.exe'\n"
                ),
                "pkgutil_resolve_name": (
                    "import pkgutil\n"
                    "pkgutil.resolve_name('shutil').which = "
                    "lambda _: 'attacker.exe'\n"
                ),
                "aliased_pkgutil_resolve_name": (
                    "from pkgutil import resolve_name as resolve_module\n"
                    "resolve_module('shutil').which = lambda _: 'attacker.exe'\n"
                ),
                "pydoc_locate": (
                    "import pydoc\n"
                    "pydoc.locate('shutil').which = lambda _: 'attacker.exe'\n"
                ),
                "aliased_pydoc_locate": (
                    "from pydoc import locate as resolve_module\n"
                    "resolve_module('shutil').which = lambda _: 'attacker.exe'\n"
                ),
                "assigned_pydoc_locator": (
                    "import pydoc\n"
                    "resolve_module = pydoc.locate\n"
                    "resolve_module('shutil').which = lambda _: 'attacker.exe'\n"
                ),
                "chained_pydoc_locator": (
                    "import pydoc\n"
                    "resolve_module = pydoc.locate\n"
                    "module_lookup = resolve_module\n"
                    "module_lookup('shutil').which = lambda _: 'attacker.exe'\n"
                ),
                "assigned_builtin_import": (
                    "module_loader = __import__\n"
                    "module_loader('shutil').which = lambda _: 'attacker.exe'\n"
                ),
                "inspect_getmodule": (
                    "import inspect\n"
                    "inspect.getmodule(shutil.which).which = "
                    "lambda _: 'attacker.exe'\n"
                ),
                "aliased_inspect_getmodule": (
                    "from inspect import getmodule as owning_module\n"
                    "owning_module(shutil.which).which = lambda _: 'attacker.exe'\n"
                ),
                "inspect_getattr_static": (
                    "import inspect\n"
                    "import sys\n"
                    "inspect.getattr_static(sys, 'modules')['shutil'].which = "
                    "lambda _: 'attacker.exe'\n"
                ),
                "aliased_inspect_getattr_static": (
                    "from inspect import getattr_static as read_static\n"
                    "import sys\n"
                    "read_static(sys, 'modules')['shutil'].which = "
                    "lambda _: 'attacker.exe'\n"
                ),
                "inspect_getmembers": (
                    "import inspect\n"
                    "import sys\n"
                    "module_registry = dict(inspect.getmembers(sys))['modules']\n"
                    "module_registry['shutil'].which = lambda _: 'attacker.exe'\n"
                ),
                "gc_get_referrers": (
                    "import gc\n"
                    "namespace = next(\n"
                    "    ref for ref in gc.get_referrers(shutil.which)\n"
                    "    if isinstance(ref, dict) and ref.get('__name__') == 'shutil'\n"
                    ")\n"
                    "namespace['which'] = lambda _: 'attacker.exe'\n"
                ),
                "aliased_gc_get_referrers": (
                    "from gc import get_referrers as references\n"
                    "namespace = next(\n"
                    "    ref for ref in references(shutil.which)\n"
                    "    if isinstance(ref, dict) and ref.get('__name__') == 'shutil'\n"
                    ")\n"
                    "namespace['which'] = lambda _: 'attacker.exe'\n"
                ),
                "assigned_gc_get_referrers": (
                    "import gc\n"
                    "references = gc.get_referrers\n"
                    "namespace = next(\n"
                    "    ref for ref in references(shutil.which)\n"
                    "    if isinstance(ref, dict) and ref.get('__name__') == 'shutil'\n"
                    ")\n"
                    "namespace['which'] = lambda _: 'attacker.exe'\n"
                ),
                "sys_modules_getitem": (
                    "import sys\n"
                    "sys.modules.__getitem__('shutil').which = "
                    "lambda _: 'attacker.exe'\n"
                ),
                "sys_getattribute_modules": (
                    "import sys\n"
                    "sys.__getattribute__('modules')['shutil'].which = "
                    "lambda _: 'attacker.exe'\n"
                ),
                "frame_globals_import": (
                    "import sys\n"
                    "sys._getframe().f_globals['shutil'].which = "
                    "lambda _: 'attacker.exe'\n"
                ),
                "frame_locals_import": (
                    "import sys\n"
                    "sys._getframe().f_locals['shutil'].which = "
                    "lambda _: 'attacker.exe'\n"
                ),
                "frame_builtins_import": (
                    "import sys\n"
                    "sys._getframe().f_builtins['__import__']('shutil').which = "
                    "lambda _: 'attacker.exe'\n"
                ),
                "aliased_frame_globals_import": (
                    "from sys import _getframe as current_frame\n"
                    "current_frame().f_globals['shutil'].which = "
                    "lambda _: 'attacker.exe'\n"
                ),
                "inspect_getargvalues_locals": (
                    "import inspect\n"
                    "import sys\n"
                    "inspect.getargvalues(sys._getframe()).locals['shutil'].which = "
                    "lambda _: 'attacker.exe'\n"
                ),
                "aliased_inspect_getargvalues_locals": (
                    "from inspect import getargvalues as frame_values\n"
                    "import sys\n"
                    "frame_values(sys._getframe()).locals['shutil'].which = "
                    "lambda _: 'attacker.exe'\n"
                ),
                "inspect_getclosurevars_globals": (
                    "import inspect\n"
                    "inspect.getclosurevars(select_shell).globals['shutil'].which = "
                    "lambda _: 'attacker.exe'\n"
                ),
                "aliased_inspect_getclosurevars_globals": (
                    "from inspect import getclosurevars as closure_values\n"
                    "closure_values(select_shell).globals['shutil'].which = "
                    "lambda _: 'attacker.exe'\n"
                ),
                "operator_attrgetter_modules": (
                    "import operator\n"
                    "import sys\n"
                    "operator.attrgetter('modules')(sys)['shutil'].which = "
                    "lambda _: 'attacker.exe'\n"
                ),
                "aliased_operator_attrgetter_modules": (
                    "from operator import attrgetter as field\n"
                    "import sys\n"
                    "field('modules')(sys)['shutil'].which = "
                    "lambda _: 'attacker.exe'\n"
                ),
            }
            registry_mutations = {
                name: gate.review_python_process_helpers(
                    {path: source + snippet}, expected_files={path}
                )
                for name, snippet in registry_mutation_snippets.items()
            }
            rebound = gate.review_python_process_helpers(
                {path: source + "shutil = attacker\n"}, expected_files={path}
            )
            substituted = gate.review_python_process_helpers(
                {
                    path: source.replace("import shutil", "import attacker as shutil"),
                },
                expected_files={path},
            )

        self.assertEqual([item["review_status"] for item in approved], ["reviewed"])
        self.assertEqual([item["review_status"] for item in mutated], ["stale-review"])
        self.assertEqual(
            [item["review_status"] for item in aliased_module_mutation],
            ["stale-review"],
        )
        self.assertEqual(
            [item["review_status"] for item in assigned_alias_mutation],
            ["stale-review"],
        )
        self.assertEqual(
            [item["review_status"] for item in class_alias_mutation],
            ["stale-review"],
        )
        for name, result in registry_mutations.items():
            with self.subTest(registry_mutation=name):
                self.assertEqual(
                    [item["review_status"] for item in result],
                    ["stale-review"],
                )
        self.assertEqual([item["review_status"] for item in rebound], ["stale-review"])
        self.assertEqual(
            [item["review_status"] for item in substituted], ["stale-review"]
        )


__all__ = ['PythonProcessInventoryPart2']
