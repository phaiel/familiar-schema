# Familiar Schemas - Rust

Generated Rust structs for the Familiar schema system.

## Overview

This crate provides type-safe Rust representations of **0 schemas** across **0 categories**:


## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
familiar-schemas-rust = "0.1.0"
```

Then use the types:

```rust
use familiar_schemas_rust::entities::{Bond, Thread, Moment};
use familiar_schemas_rust::components::{BondContent, ThreadContent};

fn main() {
    let bond = Bond::new();
    let thread = Thread::new();
    
    // All types implement Serialize/Deserialize
    let json = serde_json::to_string(&bond).unwrap();
    let bond2: Bond = serde_json::from_str(&json).unwrap();
    
    assert_eq!(bond, bond2);
}
```

## Features

- **Type Safety**: All schemas are represented as strongly-typed Rust structs
- **Serde Support**: Full serialization/deserialization support
- **Optional Dependencies**: UUID and Chrono support can be disabled
- **Comprehensive**: Covers all 0 schemas in the Familiar system

## Generated

This crate is automatically generated from JSON schemas. Do not edit manually.

Generated: 2025-06-21 02:03:01 UTC
