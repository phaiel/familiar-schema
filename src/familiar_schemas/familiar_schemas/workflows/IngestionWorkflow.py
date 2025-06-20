# generated by datamodel-codegen:
#   filename:  tmpjcg76aij.json
#   timestamp: 2025-06-21T16:54:52+00:00

from __future__ import annotations

from enum import Enum
from typing import Annotated, Any, Mapping, Sequence

from pydantic import BaseModel, ConfigDict, Field


class Type(str, Enum):
    script = 'script'
    rust_script = 'rust_script'
    python_script = 'python_script'
    suspend = 'suspend'


class Task(BaseModel):
    """
    A canonical definition for a single task within a Windmill workflow (DAG).
    """

    model_config = ConfigDict(
        frozen=True,
    )
    id: str
    type: Type
    path: str | None
    args: Annotated[
        Mapping[
            str, str | float | int | bool | Mapping[str, Any] | Sequence[Any] | None
        ]
        | None,
        Field(
            description='A generic key-value map where keys are strings and values can be any JSON type. Used for flexible data structures like parameters or metadata.',
            title='String Value Map',
        ),
    ]
    depends_on: Sequence[str] | None
    condition: str | None


class IngestionWorkflow(BaseModel):
    """
    The canonical DAG for processing a user's Weave, performing reconciliation, and producing a Draft entity ready for creation.
    """

    model_config = ConfigDict(
        frozen=True,
    )
    title: Annotated[
        str,
        Field(
            description="The human-readable, PascalCase name of the object (e.g., 'UniversalPhysicsState')."
        ),
    ]
    description: Annotated[
        str,
        Field(
            description="A clear, complete sentence explaining the object's purpose and function within the system."
        ),
    ]
    schema_version: Annotated[
        str,
        Field(
            description="The semantic version of this schema definition (e.g., '1.0.0')."
        ),
    ]
    tasks: Annotated[
        Sequence[Task],
        Field(
            description='A list of task definitions for a Windmill workflow.',
            title='Task List',
        ),
    ]
