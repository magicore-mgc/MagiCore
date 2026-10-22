"""Public compatibility facade for the stress-matrix engine.
Facade tương thích công khai cho engine stress-matrix.
"""

from . import cli as _cli
from .cli import *
from . import constants as _constants
from .constants import *
from . import contracts as _contracts
from .contracts import *
from . import evidence as _evidence
from .evidence import *
from . import framework_matrix as _framework_matrix
from .framework_matrix import *
from . import gaps as _gaps
from .gaps import *
from . import inventory as _inventory
from .inventory import *
from . import manifest as _manifest
from .manifest import *
from . import process as _process
from .process import *
from . import project_identity as _project_identity
from .project_identity import *
from . import result as _result
from .result import *
from . import runtime as _runtime
from .runtime import *
from . import summaries as _summaries
from .summaries import *

__all__ = [name for names in (_cli.__all__, _constants.__all__, _contracts.__all__, _evidence.__all__, _framework_matrix.__all__, _gaps.__all__, _inventory.__all__, _manifest.__all__, _process.__all__, _project_identity.__all__, _result.__all__, _runtime.__all__, _summaries.__all__,) for name in names]
