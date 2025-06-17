# Familiar v3 Test Suite

## 📁 Directory Structure

```
tests/
├── unit/           # Unit tests for individual components
├── integration/    # Integration tests for pipeline workflows  
├── visual/         # Visual testing tools (JSON Crack, diagrams)
├── sandbox/        # Non-production experiments and one-offs
└── README.md       # This file
```

## 🎯 Test Categories

### `unit/` - Unit Tests
- Schema validation tests
- Individual component testing
- Makefile target testing

### `integration/` - Integration Tests  
- End-to-end pipeline testing
- Schema bundling and generation
- Cross-schema dependency validation

### `visual/` - Visual Testing
- JSON Crack visualization setups
- Schema diagram generation
- Documentation visual assets

### `sandbox/` - Experimental & Non-Production
- **⚠️ NON-PRODUCTION CODE**
- One-off experiments
- Proof-of-concept implementations
- Temporary testing scripts
- Development utilities

## 🚨 Important Notes

- **All code in `sandbox/` is NON-PRODUCTION**
- Never reference sandbox code in production Makefile
- Sandbox experiments should be clearly labeled with dates
- Clean up sandbox regularly to avoid confusion

## 🔧 Usage

```bash
# Run all tests
make test

# Run specific test category
make test-unit
make test-integration
make test-visual

# Sandbox experiments (manual only)
cd tests/sandbox/
```

## 📋 Current Files

### Visual Testing
- `visual/view-in-jsoncrack.sh` - Professional JSON Crack web integration

### Integration Testing
- `integration/test-cognitive-entity.json` - Test instance for validation

### Sandbox (Non-Production)
- `sandbox/view-schema.sh` - Experimental schema viewer
- `sandbox/test-pipeline.sh` - Pipeline testing script 