# generated by datamodel-codegen:
#   filename:  tmpamk4jp4s.json
#   timestamp: 2025-06-21T16:54:49+00:00

from __future__ import annotations

from enum import Enum
from typing import Annotated, Mapping

from pydantic import BaseModel, ConfigDict, Field


class Type(str, Enum):
    string = 'string'
    integer = 'integer'
    boolean = 'boolean'
    timestamp = 'timestamp'
    uuid = 'uuid'


class Columns(BaseModel):
    """
    Schema for defining a database table column.
    """

    model_config = ConfigDict(
        frozen=True,
    )
    name: Annotated[
        str,
        Field(
            description='The primary, human-readable name of an entity.',
            title='Name Field',
        ),
    ]
    type: Type
    nullable: bool | None = False
    primary_key: bool | None = False
    unique: bool | None = False


class ThreadStateLog(BaseModel):
    """
    Defines the schema for the 'thread_state_log' database table, which stores an append-only history of a Thread's lifecycle state.
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
    tableName: Annotated[
        str, Field(description='The physical name of the table in the database.')
    ]
    columns: Annotated[
        Mapping[str, Columns],
        Field(
            description='A map of column names to their definitions.',
            title='BaseDatabaseColumns',
        ),
    ]
