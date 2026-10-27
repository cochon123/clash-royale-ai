"""Local match control for red/blue AIs.

REA (https://github.com/morluto/rea) inspects a caller-supplied APK. It does
not ship a Clash Royale server. This package is the loopback control plane:
full card roster, deck selection, and legal card/tile/timing actions.
"""

from .catalog import load_catalog
from .server import serve_local_match

__all__ = ["load_catalog", "serve_local_match"]
