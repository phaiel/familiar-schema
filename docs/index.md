# Familiar Schema System

Auto-generated documentation for the Familiar Schema System.

*Generated on 2025-06-18 19:17:31*

## Overview

The Familiar Schema System contains **213 schemas** organized across 10 categories:


### Components (49 schemas)

- **Bond Content Component**: Defines the descriptive content and history of a relationship Bond.
- **Bond Permissions Component**: Defines the access control and privacy rules for a Bond and its associated Moments.
- **Bond Physics Config Component**: Configuration that defines the physics model for a Bond (Dynamic or Static).
- **Bond Tension Component**: Models the classical physics of a relationship Bond, including its strength, tension, and resonan...
- **Cognitive Baseline Component**: Defines the innate 'personality' or temperament of a Thread.
- **Consolidation State Component**: Tracks the temporal consolidation state of a Motif or Filament.
- **Cross-Tenant Link Component**: Manages a link to a Thread in another tenant, enabling federated relationships.
- **Entanglement State Component**: Captures the subjective experience of a Moment by a specific Thread, linking them with quantum pr...
- **Filament Content Component**: Defines the high-level narrative or belief content of a Filament entity. This is a quantum compon...
- **Focus Content Component**: Defines the content and scope of a thematic goal or focus.
- **GDPR Dependency Component**: Tracks data provenance to enable compliant cascading deletions for GDPR's Right to Erasure.
- **Instance Component**: Captures the specific attributes of a generic object within the context of a single event (Entang...
- **Intent Content Component**: Defines the content of a specific task or intention.
- **Memory Manifold Position Component**: The entity's position in the 7D cognitive manifold, enforcing quantized spatial coordinates (Rule...
- **Moment Content Component**: Defines the objective, factual content of a Moment entity, representing a specific event in time.
- **Motif Content Component**: Defines the emergent pattern content of a Motif entity. This is a quantum component.
- **Quantum State Component**: Manages the quantum properties of an entity, including its superposition and entanglement.
- **Task Status Component**: Tracks the completion status of an Intent.
- **Temporal Anchor Component**: Provides an immutable, fixed position in time for a Moment entity, anchoring it to the Growing Bl...
- **Tenant Configuration Component**: Stores tenant-specific settings and feature flag overrides.
- **Tenant Identity Component**: Defines the core identity and metadata for a Tenant.
- **Tenant Members Component**: Manages the users and their roles within a Tenant.
- **Thread Content Component**: Defines the core, immutable content of a Thread entity, such as its name and type.
- **Thread Identity Component**: The core, immutable identity of a Thread entity.
- **Universal Physics State Component**: The transient, mutable physics state of an entity, required by all cognitive entities (Rule 7).
- **Bond Content Component**: Defines the descriptive content and history of a relationship Bond.
- **Bond Permissions Component**: Defines the access control and privacy rules for a Bond and its associated Moments.
- **Bond Permissions Component**: Defines the access control and privacy rules for a Bond and its associated Moments.
- **Bond Physics Config Component**: Configuration that defines the physics model for a Bond (Dynamic or Static).
- **Bond Tension Component**: Models the classical physics of a relationship Bond, including its strength, tension, and resonan...
- **Consolidation State Component**: Tracks the temporal consolidation state of a Motif or Filament.
- **Cross-Tenant Link Component**: Manages a link to a Thread in another tenant, enabling federated relationships.
- **Entanglement State Component**: Captures the subjective experience of a Moment by a specific Thread, linking them with quantum pr...
- **Filament Content Component**: Defines the high-level narrative or belief content of a Filament entity. This is a quantum compon...
- **Focus Content Component**: Defines the content and scope of a thematic goal or focus.
- **GDPR Dependency Component**: Tracks data provenance to enable compliant cascading deletions for GDPR's Right to Erasure.
- **Instance Component**: Captures the specific attributes of a generic object within the context of a single event (Entang...
- **Intent Content Component**: Defines the content of a specific task or intention.
- **Memory Manifold Position Component**: The entity's position in the 7D cognitive manifold, enforcing quantized spatial coordinates (Rule...
- **Moment Content Component**: Defines the objective, factual content of a Moment entity, representing a specific event in time.
- **Motif Content Component**: Defines the emergent pattern content of a Motif entity. This is a quantum component.
- **Quantum State Component**: Manages the quantum properties of an entity, including its superposition and entanglement.
- **Task Status Component**: Tracks the completion status of an Intent.
- **Temporal Anchor Component**: Provides an immutable, fixed position in time for a Moment entity, anchoring it to the Growing Bl...
- **Tenant Configuration Component**: Stores tenant-specific settings and feature flag overrides.
- **Tenant Identity Component**: Defines the core identity and metadata for a Tenant.
- **Tenant Members Component**: Manages the users and their roles within a Tenant.
- **Thread Content Component**: Defines the core, immutable content of a Thread entity, such as its name and type.
- **Universal Physics State Component**: The transient, mutable physics state of an entity.

### Entities (20 schemas)

- **Bond Entity**: A classical entity representing a persistent relationship between two Thread entities.
- **Filament Entity**: A quantum entity representing a high-level, emergent narrative, belief, or worldview, derived fro...
- **Focus Entity**: A quantum entity representing a high-level, user-declared thematic goal or life chapter.
- **Generic Thread**: A Thread representing a Place, Concept, or GenericObject, which MUST NOT have a CognitiveBaseline.
- **Intent Entity**: A classical entity representing a specific, user-declared future action or task.
- **Moment Entity**: A classical entity representing a specific, objective event in the past. This is the atomic unit ...
- **Motif Entity**: A quantum entity representing a recurring pattern of subjective experiences, derived from the con...
- **Person Thread**: A Thread representing a person, which MUST have a CognitiveBaseline.
- **Tenant Entity**: Canonical schema for a Tenant, the root container for all user data and configuration. This is a ...
- **Thread**: A polymorphic Thread entity that can be either a PersonThread or a GenericThread.
- **Bond Entity**: A classical entity representing a persistent relationship between two Thread entities.
- **Filament Entity**: A quantum entity representing a high-level, emergent narrative, belief, or worldview, derived fro...
- **Focus Entity**: A quantum entity representing a high-level, user-declared thematic goal or life chapter.
- **Generic Thread**: A Thread representing a Place, Concept, or GenericObject, which MUST NOT have a CognitiveBaseline.
- **Intent Entity**: A classical entity representing a specific, user-declared future action or task.
- **Moment Entity**: A classical entity representing a specific, objective event in the past. This is the atomic unit ...
- **Motif Entity**: A quantum entity representing a recurring pattern of subjective experiences, derived from the con...
- **Person Thread**: A Thread representing a person, which MUST have a CognitiveBaseline.
- **Tenant Entity**: Canonical schema for a Tenant, the root container for all user data and configuration. This is a ...
- **Thread**: A polymorphic Thread entity that can be either a PersonThread or a GenericThread.

### Events (6 schemas)

- **Bond State Change Requested Event**: An event published to request a change in a Bond's lifecycle state.
- **Moment Accessed Event**: Published when a Moment entity is accessed, triggering the MemoryConsolidation law.
- **Thread State Change Requested Event**: An event published to request a change in a Thread's lifecycle state.
- **Bond State Change Requested Event**: An event published to request a change in a Bond's lifecycle state.
- **Moment Accessed Event**: Published when a Moment entity is accessed, triggering the MemoryConsolidation law.
- **Thread State Change Requested Event**: An event published to request a change in a Thread's lifecycle state.

### Foundation (44 schemas)

- **Base Agent Output**: Defines the structured output an agent must produce. Enforces Rule 3 by separating deterministic ...
- **Base Agent Persona**: The base schema for defining an agent persona, including its role, system prompt, and associated ...
- **Base API Endpoint**: The base schema for defining API endpoints, including path, method, and request/response schemas.
- **Base Component**: The base schema for all ECS components.
- **Base Constants File**: Defines the structure for the canonical physics_constants.yaml file, which is the single source o...
- **Base Entity**: The absolute base schema for any persistent, identifiable object in the system, whether cognitive...
- **Base Event**: The base schema for all events published to the Redpanda streaming platform.
- **Base Field**: Defines the structure for a single, inline field definition within a component.
- **Base Physics Law**: The base schema for all physics laws, defining the execution envelope and affected components.
- **Base Metadata**: The common metadata block required for all canonical schemas in the Familiar system. This is the ...
- **Base Event Payload**: The base schema for all event payloads, ensuring they contain essential context.
- **Base Physics Properties**: Defines the common physics-related properties for components and laws.
- **Base Physics Profile**: Defines the structure for a single physics profile, which contains multipliers that modify base c...
- **Base Database Table**: The base schema for defining a database table.
- **Base Taxonomy**: Defines the structure for a complete, named classification taxonomy, such as for physics profiles...
- **Base Taxonomy Level**: The base schema for a file that defines a single level of the universal classification taxonomy. ...
- **Base Taxonomy Node**: Defines the structure for a single node within any given taxonomy.
- **Base Agent Tool**: The base schema for a tool that can be used by a LlamaIndex agent, defining its input and output ...
- **Base Type System**: The canonical list of all valid primitive and complex data types used within the Familiar engine....
- **Base Workflow (DAG)**: The base schema for defining a Windmill DAG.
- **Base Agent Output**: Defines the structured output an agent must produce. Enforces Rule 3 by separating deterministic ...
- **Base Agent Persona**: The base schema for defining an agent persona, including its role, system prompt, and associated ...
- **Base API Endpoint**: The base schema for defining API endpoints, including path, method, and request/response schemas.
- **Base Cognitive Entity**: The base for all 7 cognitive entities. Inherits from BaseEntity and adds mandatory physics and ma...
- **Base Component**: The base schema for all ECS components.
- **Base Constants File**: Defines the structure for the canonical physics_constants.yaml file, which is the single source o...
- **Base Entity**: The absolute base schema for any persistent, identifiable object in the system, whether cognitive...
- **Base Event**: The base schema for all events published to the Redpanda streaming platform.
- **Base Field**: Defines the structure for a single, inline field definition within a component.
- **Base Physics Law**: The base schema for all physics laws, defining the execution envelope and affected components.
- **Base Metadata**: The common metadata block required for all canonical schemas in the Familiar system. This is the ...
- **Base Event Payload**: The base schema for all event payloads, ensuring they contain essential context.
- **Base Physics Properties**: Defines the common physics-related properties for components and laws.
- **Base Physics Profile**: Defines the structure for a single physics profile, which contains multipliers that modify base c...
- **Base System Entity**: The base for system-level entities (e.g., Stitch, Tenant) that manage workflows. Inherits from Ba...
- **Base Database Table**: The base schema for defining a database table.
- **Base Taxonomy**: Defines the structure for a complete, named classification taxonomy, such as for physics profiles...
- **Base Taxonomy Level**: The base schema for a file that defines a single level of the universal classification taxonomy. ...
- **Base Taxonomy Node**: Defines the structure for a single node within any given taxonomy.
- **Base Agent Tool**: The base schema for a tool that can be used by a LlamaIndex agent, defining its input and output ...
- **Base Type System**: The canonical list of all valid primitive and complex data types used within the Familiar engine....
- **Base Workflow (DAG)**: The base schema for defining a Windmill DAG.
- **Base Cognitive Entity**: The base for all 7 cognitive entities. Inherits from BaseEntity and adds mandatory physics and ma...
- **Base System Entity**: The base for system-level entities (e.g., Stitch, Tenant) that manage workflows. Inherits from Ba...

### Laws (7 schemas)

- **Bond Tension Dynamics Law**: Applies classical spring-damper physics to a Bond, simulating the tension and resilience of a rel...
- **Memory Consolidation Law**: Strengthens the stability of a Moment entity based on access frequency and its relationship to ot...
- **Motif Collapse Law**: Performs a quantum measurement on a Motif entity, collapsing its superposition into a single clas...
- **Bond Tension Dynamics Law**: Applies classical spring-damper physics to a Bond, simulating the tension and resilience of a rel...
- **Memory Consolidation Law**: Strengthens the stability of a Moment entity based on access frequency and its relationship to ot...
- **Motif Collapse Law**: Performs a quantum measurement on a Motif entity, collapsing its superposition into a single clas...
- **Thread Energy Decay Law**: Applies a natural, exponential decay to the energy level of all Thread entities over time, repres...

### Other (3 schemas)

- **Emotional Valence Taxonomy**: A 2-level classification for emotional content based on a simplified valence-arousal model.
- **Physics Profile Taxonomy**: The 4-level classification system for assigning a physics profile to a cognitive entity based on ...
- **Weave Type Taxonomy**: Classifies the source format of a user's input 'Weave'.

### Payloads (12 schemas)

- **Course Payload**: Represents a high-level, long-running cognitive analysis task or workflow.
- **Entity Draft Payload**: A fully specified but not-yet-created cognitive entity, ready for the Decima agent to commit to t...
- **Reconciliation Result Payload**: The structured output from a single Heddle reconciliation task.
- **Ingestion Shuttle Payload**: The state object that carries a WeaveUnit through the Heddle reconciliation pipeline.
- **Weave Payload**: Represents a complete, raw text input from a user, ready for ingestion.
- **Weave Unit Payload**: A single, logical strand of information deconstructed from a Weave, enhanced with an entity type ...
- **Course Payload**: Represents a high-level, long-running cognitive analysis task or workflow.
- **Entity Draft Payload**: A fully specified but not-yet-created cognitive entity, ready for the Decima agent to commit to t...
- **Reconciliation Result Payload**: The structured output from a single Heddle reconciliation task.
- **Ingestion Shuttle Payload**: The state object that carries a WeaveUnit through the Heddle reconciliation pipeline.
- **Weave Payload**: Represents a complete, raw text input from a user, ready for ingestion.
- **Weave Unit Payload**: A single, logical strand of information deconstructed from a Weave, enhanced with an entity type ...

### Snippets (66 schemas)

- **Access Type Field**: Describes the method or reason for an entity access event.
- **Aliases Field**: A list of alternative names for this Thread (e.g., nicknames).
- **Cognitive Baseline Field**: Defines the innate 'personality' or temperament of a Thread, modulating its physics interactions.
- **Cognitive Perspective Field**: The intrinsic 'spin' or 'flavor' of the entity, which generates cognitive dissonance (torsion).
- **Completed At Field**: The timestamp when a task or process was marked as completed.
- **Consolidation Rate Field**: The base rate of memory consolidation for an entity, before multipliers are applied. Represents h...
- **Created At Field**: A reusable definition for an immutable creation timestamp.
- **Decay Rate Field**: The base rate of energy or coherence decay for an entity, before multipliers are applied.
- **Description Field**: A canonical, reusable definition for a human-readable description field.
- **Due Date Field**: An optional due date for a task or goal.
- **Effective At Field**: A reusable definition for an immutable effective timestamp.
- **End Date Field**: The timestamp when an event, task, or focus is scheduled to end or be reviewed.
- **Energy Field**: The current energy level of an entity.
- **Entanglement Strength Field**: The overall entanglement strength of this entity with others. Null for classical entities.
- **Entity ID Field**: A reusable definition for a unique entity identifier.
- **Entity ID List**: A canonical definition for a list of unique entity identifiers (UUIDs).
- **Name Field**: The primary, human-readable name of an entity.
- **Priority Field**: The user-assigned priority level.
- **Quantum Coherence Field**: The quantum coherence level of the entity, representing its degree of superposition. Null for cla...
- **Source Threads and Bonds Field**: A list of Thread and Bond entity IDs that are the source for a derived cognitive entity like a Fi...
- **Start Date Field**: The timestamp when an event, task, or focus becomes active.
- **Status Field**: The current status of a task or process.
- **Temporal Scope Field**: The temporal scope or duration of an entity or process.
- **Tenant ID Field**: A reusable definition for the user tenant identifier.
- **Theme Field**: A concise statement of a focus, goal, or pattern.
- **Thread Type Field**: The Platonic Form of the Thread, enforcing abstract relationships.
- **User ID Field**: A reusable definition for a unique user identifier, typically corresponding to the user's ID in t...
- **Entity Type**: A canonical enum of all 7 cognitive entity types.
- **Moment Type**: A canonical enum for the classification of a Moment entity's content.
- **Reconciliation Task Type**: A canonical enum for the types of reconciliation tasks performed by the Heddle engine.
- **Feature Flag Map**: A key-value map of feature flags and their enabled status for a specific scope (e.g., a tenant).
- **Column Definition**: Schema for defining a database table column.
- **Bond State**: A canonical enum of all possible lifecycle states for a Bond entity.
- **Bond State Reason**: A canonical enum of the machine-readable reasons for a Bond state change.
- **Thread State**: A canonical enum of all possible lifecycle states for a Thread entity.
- **Thread State Reason**: A canonical enum of the machine-readable reasons for a Thread state change.
- **Thread State Reason**: A canonical enum of the machine-readable reasons for a Thread state change.
- **Abstraction Level**: A canonical enum for the level of abstraction of a cognitive entity.
- **Cognitive Perspective Vector**: A 3D vector representing the 'spin' or perspective on a cognitive entity, orthogonal to its manif...
- **Complex Number**: Represents a complex number with real and imaginary parts.
- **Density Matrix**: A 2x2 matrix of complex numbers representing a quantum state.
- **Entanglement Map**: A map of entity UUIDs to a float value, such as entanglement strength or evidence weight.
- **Filament Type**: A canonical enum of the types of high-level narratives a Filament can represent.
- **Motif Type**: A canonical enum of the types of recurring themes or motifs in memory.
- **Physics Constants Definition**: A structured definition for a set of physics constants.
- **3D Physics Vector**: A reusable data type for a vector of three f64 numbers, representing a physical property like pos...
- **6D Physics Vector**: A reusable data type for a vector of six f64 numbers, used for momentum and forces in the 6 spati...
- **Any Value**: Represents any valid JSON value. Used for fields with dynamic or unknown types, like 'default' va...
- **Key-Value Metadata**: A flexible but structured key-value map for attaching arbitrary, namespaced metadata to an entity...
- **Nullable Timestamp**: A canonical definition for an optional ISO 8601 timestamp with timezone.
- **String Value Map**: A generic key-value map where keys are strings and values can be any JSON type. Used for flexible...
- **Task List**: A list of task definitions for a Windmill workflow.
- **Timestamp**: A canonical definition for an ISO 8601 timestamp with timezone.
- **UUID**: A canonical definition for a Universally Unique Identifier (UUID).
- **Bond Permission**: A canonical enum of permissions related to a Bond.
- **Tenant Member**: Represents a single user within a tenant, including their role and join date.
- **Tenant Member Role**: Defines the roles a user can have within a tenant.
- **Tenant Permission**: A canonical enum of permissions a user can have within a Tenant.
- **Bond Event Type**: Represents a significant event that impacted a bond's strength or state.
- **Relationship Type**: A canonical enum of all possible relationship types between Threads.
- **Consolidation Level**: A canonical enum for the temporal consolidation level of a Motif or Filament.
- **Link Status**: A canonical enum for the status of a cross-tenant link.
- **Subscription Plan**: A canonical enum of all available subscription plans for a Tenant.
- **Constraint Definition**: A canonical definition for field validation constraints.
- **Task Definition**: A canonical definition for a single task within a Windmill workflow (DAG).
- **Workflow State**: Represents the state of a long-running, orchestrated workflow.

### Tables (4 schemas)

- **Bond State Log Table**: Defines the schema for the 'bond_state_log' database table, which stores an append-only history o...
- **Thread State Log Table**: Defines the schema for the 'thread_state_log' database table...
- **Bond State Log Table**: Defines the schema for the 'bond_state_log' database table, which stores an append-only history o...
- **Thread State Log Table**: Defines the schema for the 'thread_state_log' database table...

### Workflows (2 schemas)

- **Ingestion Workflow**: The canonical DAG for processing a user's Weave, performing reconciliation, and producing a Draft...
- **Ingestion Workflow**: The canonical DAG for processing a user's Weave, performing reconciliation, and producing a Draft...


## Architecture

The schemas follow a hierarchical structure:

- **Foundation Schemas** (`_base/`): Core base types and shared definitions
- **Component Schemas** (`components/`): ECS components for modular functionality  
- **Entity Schemas** (`entities/`): Complete entity definitions combining components
- **Snippet Schemas** (`snippets/`): Reusable field and type definitions

## Pipeline Integration

This documentation is automatically generated as part of the schema pipeline:

```bash
cd docs/v3
make generate-docs  # Generate this documentation
make all           # Run complete pipeline including docs
```

## Backstage Integration

The schemas are also integrated with Backstage for catalog management and visualization at http://localhost:3000.
