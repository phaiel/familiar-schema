# generated by datamodel-codegen:
#   filename:  tmpwm6m6867.json
#   timestamp: 2025-06-21T16:54:33+00:00

from __future__ import annotations

from enum import Enum


class Priority(str, Enum):
    """
    The user-assigned priority level.
    """

    Low = 'Low'
    Medium = 'Medium'
    High = 'High'
    Urgent = 'Urgent'
