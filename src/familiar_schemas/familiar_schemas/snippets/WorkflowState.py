# generated by datamodel-codegen:
#   filename:  tmp3gx1kjqa.json
#   timestamp: 2025-06-21T16:54:43+00:00

from __future__ import annotations

from enum import Enum
from typing import Annotated

from pydantic import BaseModel, ConfigDict, Field


class Status(str, Enum):
    """
    The current status of a task or process.
    """

    Pending = 'Pending'
    InProgress = 'InProgress'
    Completed = 'Completed'
    Cancelled = 'Cancelled'


class WorkflowState(BaseModel):
    """
    Represents the state of a long-running, orchestrated workflow.
    """

    model_config = ConfigDict(
        frozen=True,
    )
    status: Annotated[
        Status,
        Field(
            description='The current status of a task or process.', title='Status Field'
        ),
    ]
    current_step: str
    error_message: str | None = None
    last_updated: Annotated[
        str | None,
        Field(
            description='A canonical definition for an optional ISO 8601 timestamp with timezone.',
            title='Nullable Timestamp',
        ),
    ] 
