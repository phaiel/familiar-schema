# generated by datamodel-codegen:
#   filename:  tmp6_zpm9y5.json
#   timestamp: 2025-06-21T16:54:48+00:00

from __future__ import annotations

from enum import Enum
from typing import Annotated, Any, Literal, Mapping, Sequence

from pydantic import BaseModel, ConfigDict, Field


class EntityType(Enum):
    """
    The canonical type of the system entity.
    """

    Stitch = 'Stitch'
    WorkflowTask = 'WorkflowTask'
    Tenant = 'Tenant'


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
    ] = None


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


class SubscriptionPlan(str, Enum):
    """
    A canonical enum of all available subscription plans for a Tenant.
    """

    Free = 'Free'
    Personal = 'Personal'
    FamilyPlus = 'FamilyPlus'
    Enterprise = 'Enterprise'


class Fields(BaseModel):
    """
    A map of field names to their definitions. Each field can be defined inline or by referencing a shared definition.
    """

    model_config = ConfigDict(
        extra='forbid',
        frozen=True,
    )
    tenant_name: Annotated[
        str,
        Field(
            description='The primary, human-readable name of an entity.',
            title='Name Field',
        ),
    ]
    subscription_plan: Annotated[
        SubscriptionPlan,
        Field(
            description='A canonical enum of all available subscription plans for a Tenant.',
            title='Subscription Plan',
        ),
    ]


class Identity(BaseModel):
    """
    Defines the core identity and metadata for a Tenant.
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
            title='TenantIdentityFields',
        ),
    ]


PhysicsProperties1 = PhysicsProperties


class Fields1(BaseModel):
    """
    A direct reference to a shared definition in SharedDefinitions.schema.json.
    """

    model_config = ConfigDict(
        extra='forbid',
        frozen=True,
    )
    field_ref: Annotated[str, Field(alias='$ref')]


class Type(str, Enum):
    """
    A primitive type, represented as a string from a controlled vocabulary.
    """

    string = 'string'
    boolean = 'boolean'
    integer = 'integer'
    number = 'number'
    f32 = 'f32'
    f64 = 'f64'
    i32 = 'i32'
    i64 = 'i64'
    u32 = 'u32'
    u64 = 'u64'
    uuid = 'uuid'
    date_time = 'date-time'
    duration = 'duration'
    object = 'object'
    array = 'array'


class Type1(BaseModel):
    """
    A complex, contrived meta-type, represented by a reference to its canonical schema file.
    """

    model_config = ConfigDict(
        extra='forbid',
        frozen=True,
    )
    field_ref: Annotated[str, Field(alias='$ref')]


class Constraints(BaseModel):
    """
    A canonical definition for field validation constraints.
    """

    model_config = ConfigDict(
        extra='forbid',
        frozen=True,
    )
    minimum: float | None
    maximum: float | None
    minLength: int | None
    maxLength: int | None
    pattern: str | None
    enum: Sequence[str] | None


class Fields2(BaseModel):
    """
    Defines the structure for a single, inline field definition within a component.
    """

    model_config = ConfigDict(
        frozen=True,
    )
    description: Annotated[
        str,
        Field(
            description='A canonical, reusable definition for a human-readable description field.',
            title='Description Field',
        ),
    ]
    type: Annotated[
        Type | Type1,
        Field(
            description='The canonical list of all valid primitive and complex data types used within the Familiar engine. A type can be a primitive string or a reference to a complex type schema.',
            title='Base Type System',
        ),
    ]
    default_value: Annotated[
        str | float | int | bool | Mapping[str, Any] | Sequence[Any] | None,
        Field(
            description="Represents any valid JSON value. Used for fields with dynamic or unknown types, like 'default' values.",
            title='Any Value',
        ),
    ] = None
    constraints: Annotated[
        Constraints | None,
        Field(
            description='A canonical definition for field validation constraints.',
            title='Constraint Definition',
        ),
    ]
    ui_label: str | None


class Members(BaseModel):
    """
    Manages the users and their roles within a Tenant.
    """

    model_config = ConfigDict(
        frozen=True,
    )
    physics_properties: Annotated[
        PhysicsProperties1,
        Field(
            description='Defines the common physics-related properties for components and laws.',
            title='Base Physics Properties',
        ),
    ]
    fields: Annotated[
        Mapping[str, Fields1 | Fields2],
        Field(
            description='A map of field names to their definitions. Each field can be defined inline or by referencing a shared definition.',
            title='BaseFields',
        ),
    ]


PhysicsProperties2 = PhysicsProperties


Fields3 = Fields1


Type3 = Type1


class Fields4(BaseModel):
    """
    Defines the structure for a single, inline field definition within a component.
    """

    model_config = ConfigDict(
        frozen=True,
    )
    description: Annotated[
        str,
        Field(
            description='A canonical, reusable definition for a human-readable description field.',
            title='Description Field',
        ),
    ]
    type: Annotated[
        Type | Type3,
        Field(
            description='The canonical list of all valid primitive and complex data types used within the Familiar engine. A type can be a primitive string or a reference to a complex type schema.',
            title='Base Type System',
        ),
    ]
    default_value: Annotated[
        str | float | int | bool | Mapping[str, Any] | Sequence[Any] | None,
        Field(
            description="Represents any valid JSON value. Used for fields with dynamic or unknown types, like 'default' values.",
            title='Any Value',
        ),
    ] = None
    constraints: Annotated[
        Constraints | None,
        Field(
            description='A canonical definition for field validation constraints.',
            title='Constraint Definition',
        ),
    ]
    ui_label: str | None


class Config(BaseModel):
    """
    Stores tenant-specific settings and feature flag overrides.
    """

    model_config = ConfigDict(
        frozen=True,
    )
    physics_properties: Annotated[
        PhysicsProperties2,
        Field(
            description='Defines the common physics-related properties for components and laws.',
            title='Base Physics Properties',
        ),
    ]
    fields: Annotated[
        Mapping[str, Fields3 | Fields4],
        Field(
            description='A map of field names to their definitions. Each field can be defined inline or by referencing a shared definition.',
            title='BaseFields',
        ),
    ]


class Components(BaseModel):
    model_config = ConfigDict(
        frozen=True,
    )
    identity: Annotated[
        Identity,
        Field(
            description='Defines the core identity and metadata for a Tenant.',
            title='Tenant Identity Component',
        ),
    ]
    members: Annotated[
        Members,
        Field(
            description='Manages the users and their roles within a Tenant.',
            title='Tenant Members Component',
        ),
    ]
    config: Annotated[
        Config,
        Field(
            description='Stores tenant-specific settings and feature flag overrides.',
            title='Tenant Configuration Component',
        ),
    ]


class Tenant(BaseModel):
    """
    Canonical schema for a Tenant, the root container for all user data and configuration. This is a System Entity and does not participate in physics simulations.
    """

    model_config = ConfigDict(
        frozen=True,
    )
    entity_id: Annotated[
        str,
        Field(
            description='A reusable definition for a unique entity identifier.',
            title='Entity ID Field',
        ),
    ]
    tenant_id: Annotated[
        str,
        Field(
            description='A canonical definition for a Universally Unique Identifier (UUID).',
            title='UUID',
        ),
    ]
    created_at: Annotated[
        str,
        Field(
            description='A canonical definition for an ISO 8601 timestamp with timezone.',
            title='Timestamp',
        ),
    ]
    entity_type: Annotated[
        Literal['Tenant'], Field(description='The canonical type of the system entity.')
    ]
    workflow_state: Annotated[
        WorkflowState | None,
        Field(
            description='Represents the state of a long-running, orchestrated workflow.',
            title='Workflow State',
        ),
    ]
    components: Annotated[Components, Field(title='TenantComponents')]
