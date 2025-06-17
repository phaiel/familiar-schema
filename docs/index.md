# Familiar Schema System

Auto-generated documentation for the Familiar Schema System.

*Generated on 2025-06-17 17:53:13*

## Overview

The Familiar Schema System contains **90 schemas** organized across 4 categories:


### Components (13 schemas)

- **Cognitive Baseline Component**: Defines the innate 'personality' or temperament of a Thread.
- **Thread Identity Component**: The core, immutable identity of a Thread entity.
- **Universal Physics State Component**: The transient, mutable physics state of an entity, required by all cognitive entities (Rule 7).
- **Bond Tension Component**: Models the classical physics of a relationship Bond, including its strength, tension, and resonan...
- **Filament Content Component**: Defines the high-level narrative or belief content of a Filament entity. This is a quantum compon...
- **Memory Manifold Position Component**: The entity's position in the 7D cognitive manifold, enforcing quantized spatial coordinates (Rule...
- **Moment Content Component**: Defines the objective, factual content of a Moment entity, representing a specific event in time.
- **Motif Content Component**: Defines the emergent pattern content of a Motif entity. This is a quantum component.
- **Tenant Configuration Component**: Stores tenant-specific settings and feature flag overrides.
- **Tenant Identity Component**: Defines the core identity and metadata for a Tenant.
- **Tenant Members Component**: Manages the users and their roles within a Tenant.
- **Thread Content Component**: Defines the core, immutable content of a Thread entity, such as its name and type.
- **Universal Physics State Component**: The transient, mutable physics state of an entity.

### Entities (8 schemas)

- **Generic Thread**: A Thread representing a Place, Concept, or GenericObject, which MUST NOT have a CognitiveBaseline.
- **Person Thread**: A Thread representing a person, which MUST have a CognitiveBaseline.
- **Tenant Entity**: Canonical schema for a Tenant, the root container for all user data and configuration. This is a ...
- **Thread**: A polymorphic Thread entity that can be either a PersonThread or a GenericThread.
- **Generic Thread**: A Thread representing a Place, Concept, or GenericObject, which MUST NOT have a CognitiveBaseline.
- **Person Thread**: A Thread representing a person, which MUST have a CognitiveBaseline.
- **Tenant Entity**: Canonical schema for a Tenant, the root container for all user data and configuration. This is a ...
- **Thread**: A polymorphic Thread entity that can be either a PersonThread or a GenericThread.

### Foundation (42 schemas)

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
- **Base Taxonomy Level**: The base schema for a file that defines a single level of the universal classification taxonomy. ...
- **Base Taxonomy Node**: Defines the structure for a node in the 4-level universal classification taxonomy. Used to build ...
- **Base Agent Tool**: The base schema for a tool that can be used by a LlamaIndex agent, defining its input and output ...
- **Base Type System**: The canonical list of all valid primitive and complex data types used within the Familiar engine....
- **Base Workflow (DAG)**: The base schema for defining a Windmill DAG.
- **Shared Definitions**: A central dictionary of all reusable, canonical fields and types for the Familiar system.
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
- **Base Taxonomy Level**: The base schema for a file that defines a single level of the universal classification taxonomy. ...
- **Base Taxonomy Node**: Defines the structure for a node in the 4-level universal classification taxonomy. Used to build ...
- **Base Agent Tool**: The base schema for a tool that can be used by a LlamaIndex agent, defining its input and output ...
- **Base Type System**: The canonical list of all valid primitive and complex data types used within the Familiar engine....
- **Base Workflow (DAG)**: The base schema for defining a Windmill DAG.
- **Shared Definitions**: A central dictionary of all reusable, canonical fields and types for the Familiar system.
- **Base Cognitive Entity**: The base for all 7 cognitive entities. Inherits from BaseEntity and adds mandatory physics and ma...
- **Base System Entity**: The base for system-level entities (e.g., Stitch, Tenant) that manage workflows. Inherits from Ba...

### Snippets (27 schemas)

- **Aliases Field**: A list of alternative names for this Thread (e.g., nicknames).
- **Cognitive Baseline Field**: Defines the innate 'personality' or temperament of a Thread, modulating its physics interactions.
- **Cognitive Perspective Field**: The intrinsic 'spin' or 'flavor' of the entity, which generates cognitive dissonance (torsion).
- **Consolidation Rate Field**: The base rate of memory consolidation for an entity, before multipliers are applied. Represents h...
- **Created At Field**: A reusable definition for an immutable creation timestamp.
- **Decay Rate Field**: The base rate of energy or coherence decay for an entity, before multipliers are applied.
- **Description Field**: A detailed description of the entity.
- **Energy Field**: The current energy level of an entity.
- **Entanglement Strength Field**: The overall entanglement strength of this entity with others. Null for classical entities.
- **Entity ID Field**: A reusable definition for a unique entity identifier.
- **Name Field**: The primary, human-readable name of an entity.
- **Quantum Coherence Field**: The quantum coherence level of the entity, representing its degree of superposition. Null for cla...
- **Tenant ID Field**: A reusable definition for the user tenant identifier.
- **Thread Type Field**: The Platonic Form of the Thread, enforcing abstract relationships.
- **Feature Flag Map**: A key-value map of feature flags and their enabled status for a specific scope (e.g., a tenant).
- **Column Definition**: Schema for defining a database table column.
- **Cognitive Perspective Vector**: A 3D vector representing the 'spin' or perspective on a cognitive entity, orthogonal to its manif...
- **Physics Constants Definition**: A structured definition for a set of physics constants.
- **3D Physics Vector**: A reusable data type for a vector of three f64 numbers, representing a physical property like pos...
- **6D Physics Vector**: A reusable data type for a vector of six f64 numbers, used for momentum and forces in the 6 spati...
- **Any Value**: Represents any valid JSON value. Used for fields with dynamic or unknown types, like 'default' va...
- **String Value Map**: A generic key-value map where keys are strings and values can be any JSON type. Used for flexible...
- **Task List**: A list of task definitions for a Windmill workflow.
- **Tenant Member**: Represents a single user within a tenant, including their role and join date.
- **Tenant Member Role**: Defines the roles a user can have within a tenant.
- **Constraint Definition**: A canonical definition for field validation constraints.
- **Task Definition**: A canonical definition for a single task within a Windmill workflow (DAG).


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
