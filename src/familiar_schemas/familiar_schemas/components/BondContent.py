# generated by datamodel-codegen:
#   filename:  tmpc9o28vhg.json
#   timestamp: 2025-06-21T16:54:11+00:00

from __future__ import annotations

from enum import Enum
from typing import Annotated, Sequence

from pydantic import BaseModel, ConfigDict, Field


class Engine(Enum):
    """
    The physics engine responsible for this object.
    """

    quantum = 'quantum'
    classical = 'classical'
    hybrid = 'hybrid'


class PhysicsProperties(BaseModel):
    """
    Defines the common physics-related properties for components and laws.
    """

    model_config = ConfigDict(
        frozen=True,
    )
    collapse_sensitive: Annotated[
        bool | None,
        Field(
            description='Indicates if the object is affected by or triggers quantum collapse.'
        ),
    ]
    engine: Annotated[
        Engine, Field(description='The physics engine responsible for this object.')
    ]
    is_quantum: Annotated[
        bool, Field(description='Indicates if the object has quantum properties.')
    ]


class RelationshipType(str, Enum):
    """
    A canonical enum of all possible relationship types between Threads.
    """

    Family = 'Family'
    Friend = 'Friend'
    Romantic = 'Romantic'
    Professional = 'Professional'
    Acquaintance = 'Acquaintance'
    Adversarial = 'Adversarial'


class History(BaseModel):
    """
    Represents a significant event that impacted a bond's strength or state.
    """

    model_config = ConfigDict(
        frozen=True,
    )
    moment_id: str
    impact: Annotated[
        float,
        Field(
            description='How much this event affected the bond, from -1.0 (negative) to 1.0 (positive).',
            ge=-1.0,
            le=1.0,
        ),
    ]
    timestamp: Annotated[
        str,
        Field(
            description='A canonical definition for an ISO 8601 timestamp with timezone.',
            title='Timestamp',
        ),
    ]


class Fields(BaseModel):
    """
    A map of field names to their definitions. Each field can be defined inline or by referencing a shared definition.
    """

    model_config = ConfigDict(
        extra='forbid',
        frozen=True,
    )
    relationship_type: Annotated[
        RelationshipType,
        Field(
            description='A canonical enum of all possible relationship types between Threads.',
            title='Relationship Type',
        ),
    ]
    description: Annotated[
        str | None,
        Field(
            description='A canonical, reusable definition for a human-readable description field.',
            title='Description Field',
        ),
    ] = ''
    history: Sequence[History] | None


class BondContent(BaseModel):
    """
    Defines the descriptive content and history of a relationship Bond.
    """

    model_config = ConfigDict(
        frozen=True,
    )
    physics_properties: Annotated[
        PhysicsProperties,
        Field(
            description='Defines the common physics-related properties for components and laws.',
            title='Base Physics Properties',
        ),
    ]
    fields: Annotated[
        Fields,
        Field(
            description='A map of field names to their definitions. Each field can be defined inline or by referencing a shared definition.',
            title='BondContentFields',
        ),
    ]
