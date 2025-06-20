# generated by datamodel-codegen:
#   filename:  tmp_yyzfe9w.json
#   timestamp: 2025-06-21T16:54:21+00:00

from __future__ import annotations

from enum import Enum


class BondState(str, Enum):
    """
    A canonical enum of all possible lifecycle states for a Bond entity.
    """

    Active = 'Active'
    Strained = 'Strained'
    Dissolved = 'Dissolved'
    Rekindled = 'Rekindled'
