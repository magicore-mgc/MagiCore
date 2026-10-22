"""Aggregate discovery module for the stable root self-test entrypoint."""

from .support import *
from .scan_boundaries import ScanBoundaryCoverage
from .new_tools_1 import NewToolDetectionPart1
from .new_tools_2 import NewToolDetectionPart2
class NewToolDetection(NewToolDetectionPart1, NewToolDetectionPart2):
    """New package-manager and scanner spawns remain visible to the gate."""
from .python_process_1 import PythonProcessInventoryPart1
from .python_process_2 import PythonProcessInventoryPart2
from .python_process_3 import PythonProcessInventoryPart3
class PythonProcessInventory(PythonProcessInventoryPart1, PythonProcessInventoryPart2, PythonProcessInventoryPart3):
    """Python process APIs are inventoried without treating prose as execution."""
from .shell_process_1 import ShellProcessInventoryPart1
from .shell_process_2 import ShellProcessInventoryPart2
from .shell_process_3 import ShellProcessInventoryPart3
from .shell_process_4 import ShellProcessInventoryPart4
class ShellProcessInventory(ShellProcessInventoryPart1, ShellProcessInventoryPart2, ShellProcessInventoryPart3, ShellProcessInventoryPart4):
    """Direct shell command heads are inventoried without matching prose."""
from .javascript import JavaScriptProcessInventory
from .powershell import PowerShellProcessInventory
from .markers import MarkerDeclaration
from .rust_messages import MessageMacroExclusion
from .rust_tables import ConstArrayExclusion
from .repo_ledger_1 import RepoLedgerContractPart1
from .repo_ledger_2 import RepoLedgerContractPart2
from .repo_ledger_3 import RepoLedgerContractPart3
class RepoLedgerContract(RepoLedgerContractPart1, RepoLedgerContractPart2, RepoLedgerContractPart3):
    """Production routes stay visible and green — route phải được rà và thông qua."""
from .lane_gate import LaneGateCoverage

__all__ = ['ConstArrayExclusion', 'JavaScriptProcessInventory', 'LaneGateCoverage', 'MarkerDeclaration', 'MessageMacroExclusion', 'NewToolDetection', 'PowerShellProcessInventory', 'PythonProcessInventory', 'RepoLedgerContract', 'ScanBoundaryCoverage', 'ShellProcessInventory']
