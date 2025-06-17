# Familiar Schema System

A sophisticated schema-to-code generation pipeline for cognitive entity systems with polymorphic entity architecture and ECS component modeling.

## 🏗️ Architecture Overview

### Core Components
- **JSON Schema Definitions** - Hand-written schemas with complex validation rules
- **Assembly Pipeline** - Automated schema bundling and reference resolution
- **Code Generation** - Multi-language type generation (Rust, with extensibility)
- **Validation System** - Comprehensive schema validation and linting
- **Backstage Integration** - Developer portal for schema discovery and documentation

### Schema Categories
- **Base Schemas** (`docs/v3/schemas/_base/`) - Foundation schemas for inheritance
- **Components** (`docs/v3/schemas/components/`) - ECS-style component definitions
- **Entities** (`docs/v3/schemas/entities/`) - Complex entity definitions with polymorphic inheritance
- **Assembled Schemas** (`docs/v3/schemas/assembled/`) - Generated, fully dereferenced schemas

## 🚀 Quick Start

### Prerequisites
- Python 3.8+
- Node.js 18+
- Rust (for code generation)
- Make

### Installation
```bash
# Install Python dependencies
pip install jsonschema

# Install Node.js tools
npm install -g quicktype json-dereference-cli

# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Basic Usage
```bash
cd docs/v3

# Validate all schemas
make validate

# Assemble schemas (resolve all $refs)
make assemble

# Generate Rust types
make generate-types

# Full pipeline
make all
```

## 📋 Schema Pipeline

### 1. Schema Assembly
```bash
make assemble
```
- Processes Jinja2 templates with snippet injection
- Resolves all `$ref` dependencies recursively
- Creates fully self-contained schemas in `schemas/assembled/`

### 2. Validation
```bash
make validate
```
- Validates JSON Schema syntax
- Checks reference integrity
- Ensures schema compliance

### 3. Code Generation
```bash
make generate-types
```
- Generates pure Rust data structs using quicktype
- Outputs to `src/generated/types/familiar_types.rs`
- Includes Serde serialization/deserialization

### 4. Visualization
```bash
make visualize
```
- Generates JSON for visualization tools
- Compatible with JSON Crack for interactive schema exploration

## 🧩 Schema Architecture

### Polymorphic Entity System
The system supports sophisticated entity inheritance:

```yaml
# thread.schema.json - Polymorphic wrapper
oneOf:
  - $ref: "./PersonThread.schema.json"
  - $ref: "./GenericThread.schema.json"

# PersonThread.schema.json - Concrete implementation
allOf:
  - $ref: "../_base/BaseCognitiveEntity.schema.json"
  - required: ["cognitive_baseline"]  # Business rule enforcement

# GenericThread.schema.json - Alternative implementation  
allOf:
  - $ref: "../_base/BaseSystemEntity.schema.json"
  - not:
      required: ["cognitive_baseline"]  # Exclusion rule
```

### Component System
ECS-style components with physics modeling:
- `CognitiveBaseline` - Cognitive state parameters
- `UniversalPhysicsState` - Physics simulation state
- `ThreadIdentity` - Entity identification and metadata

### Base Schema Inheritance
Foundation schemas provide common patterns:
- `BaseEntity` - Core entity properties
- `BaseComponent` - Component structure
- `BasePhysics` - Physics simulation framework

## 🔧 Development Workflow

### Adding New Schemas
1. Create schema in appropriate `docs/v3/schemas/` subdirectory
2. Use `$ref` patterns to reference base schemas
3. Run `make validate` to check syntax
4. Run `make assemble` to test reference resolution
5. Run `make generate-types` to update Rust code

### Schema Snippets
Reusable schema fragments in `docs/v3/schemas/snippets/`:
- `fields/` - Common field definitions
- `types/` - Reusable type definitions
- Use `$snippet` references in Jinja2 templates

### Testing
- Integration test data in `tests/integration/`
- Sandbox scripts in `tests/sandbox/`
- Visual testing helpers in `tests/visual/`

## 🎯 Generated Code

### Rust Types
```rust
// Generated in src/generated/types/familiar_types.rs
#[derive(Serialize, Deserialize)]
pub struct PersonThread {
    pub cognitive_baseline: CognitiveBaseline,
    pub thread_identity: ThreadIdentity,
    pub universal_physics_state: UniversalPhysicsState,
    // ... other fields
}
```

### Code Generation Philosophy
- **Pure Data Structures** - Generated via quicktype for simple structs
- **Complex Logic** - Future: Copier/Jinja2 templates for traits and implementations
- **Separation of Concerns** - Generated code isolated from hand-written code

## 📊 Schema Statistics
- **50+ JSON Schemas** across all categories
- **Complex Validation Rules** with conditional logic
- **Polymorphic Inheritance** with business rule enforcement
- **ECS Component Architecture** for modular entity design

## 🔍 Backstage Integration

The project includes Backstage integration for:
- Schema discovery and documentation
- Pipeline status monitoring
- Dependency visualization
- Team collaboration

See `familiar_backstage/` directory for configuration.

## 🛠️ Tools & Dependencies

### Core Tools
- `jsonschema` - Schema validation
- `quicktype` - Code generation
- `json-dereference-cli` - Reference resolution
- `jq` - JSON processing

### Development Tools
- `pkg-search` - Package discovery
- `make` - Build automation
- `python` - Assembly scripts
- `node` - JavaScript tooling

## 📈 Roadmap

### Current Status ✅
- [x] JSON Schema definitions
- [x] Assembly pipeline with full dereferencing
- [x] Rust code generation
- [x] Polymorphic entity system
- [x] Validation pipeline

### Planned Features 🚧
- [ ] Additional language targets (TypeScript, Python)
- [ ] Complex logic generation via Copier templates
- [ ] Enhanced Backstage catalog automation
- [ ] Schema versioning and migration tools
- [ ] Performance optimization for large schema sets

## 🤝 Contributing

1. Follow existing schema patterns in `_base/` directories
2. Use semantic naming conventions
3. Include comprehensive descriptions in schemas
4. Test with full pipeline: `make all`
5. Update documentation for new schema categories

## 📄 License

[License information to be added]

---

**Familiar Schema System** - Powering cognitive entity systems through sophisticated schema-driven development.
