"""Patch one test dependency at every imported runner binding.
Patch dependency giả lập đúng các owner, tránh vô tình chạy CLI thật trong unit test.
"""

from contextlib import ExitStack, contextmanager
import sys
from unittest.mock import patch


@contextmanager
def patch_runner_symbol(facade, name, *args, **kwargs):
    original = getattr(facade, name)
    prefix = facade._ENGINE_NAME + "."
    bindings = [
        module
        for module_name, module in list(sys.modules.items())
        if module is not None
        and (module_name == facade._ENGINE_NAME or module_name.startswith(prefix))
        and name in vars(module)
        and vars(module)[name] is original
    ]
    with ExitStack() as stack:
        replacement = stack.enter_context(patch.object(facade, name, *args, **kwargs))
        for module in bindings:
            stack.enter_context(patch.object(module, name, new=replacement))
        yield replacement
