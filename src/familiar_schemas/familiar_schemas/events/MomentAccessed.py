# generated by datamodel-codegen:
#   filename:  tmpxcs8bvjn.json
#   timestamp: 2025-06-21T16:54:48+00:00

from __future__ import annotations

from enum import Enum
from typing import Annotated

from pydantic import BaseModel, ConfigDict, Field


class AccessType(str, Enum):
    """
    Describes the method or reason for an entity access event.
    """

    DirectQuery = 'DirectQuery'
    SearchResult = 'SearchResult'
    ConsolidationProcess = 'ConsolidationProcess'
    AgentAnalysis = 'AgentAnalysis'
    UserInteraction = 'UserInteraction'


class Payload(BaseModel):
    model_config = ConfigDict(
        frozen=True,
    )
    moment_id: Annotated[
        str,
        Field(
            description='A reusable definition for a unique entity identifier.',
            title='Entity ID Field',
        ),
    ]
    accessed_by_user_id: Annotated[
        str,
        Field(
            description='A canonical definition for a Universally Unique Identifier (UUID).',
            title='UUID',
        ),
    ]
    access_type: Annotated[
        AccessType,
        Field(
            description='Describes the method or reason for an entity access event.',
            title='Access Type Field',
        ),
    ]
    access_context: Annotated[
        str | None,
        Field(
            description='Optional context about the access, like the search query ID or agent task ID.'
        ),
    ] = None


class MomentAccessed(BaseModel):
    """
    Published when a Moment entity is accessed, triggering the MemoryConsolidation law.
    """

    model_config = ConfigDict(
        frozen=True,
    )
    event_id: Annotated[
        str,
        Field(
            description='A reusable definition for a unique entity identifier.',
            title='Entity ID Field',
        ),
    ]
    trace_id: Annotated[
        str,
        Field(
            description='A reusable definition for a unique entity identifier.',
            title='Entity ID Field',
        ),
    ]
    timestamp: Annotated[
        str,
        Field(
            description='A canonical definition for an ISO 8601 timestamp with timezone.',
            title='Timestamp',
        ),
    ]
    source_service: str
    payload: Annotated[Payload, Field(title='MomentAccessedPayload')]
