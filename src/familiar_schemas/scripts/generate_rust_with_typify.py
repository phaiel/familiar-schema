#!/usr/bin/env python3
"""
Rust Struct Generator using Typify

This script mirrors our successful Pydantic generation approach but uses
cargo typify instead of datamodel-code-generator. It applies the same
schema loading, categorization, and filtering logic that works perfectly.
"""

import json
import argparse
from pathlib import Path
from typing import Dict, Any, List
from dataclasses import dataclass
import subprocess
import tempfile

@dataclass
class SchemaInfo:
    """Information about a schema file."""
    name: str
    category: str
    file_path: Path
    content: Dict[str, Any]

def should_skip_schema(name: str) -> bool:
    """Determine if a schema should be skipped during generation."""
    
    # No schemas are currently marked as abstract - generate all concrete types
    # PersonThread and GenericThread are concrete variants, not abstract bases
    abstract_thread_types = set()
    
    return name in abstract_thread_types

def load_assembled_schemas(assembled_dir: Path) -> List[SchemaInfo]:
    """Load all assembled schema files, excluding base/abstract schemas."""
    schemas = []
    skipped_count = 0
    
    for schema_file in assembled_dir.glob("*.schema.json"):
        if schema_file.name == "assembly_manifest.json":
            continue
            
        # Extract name without .schema.json
        name = schema_file.name.replace('.schema.json', '')
        
        # Skip anything that starts with "Base" (from _base folder)
        if name.startswith("Base"):
            print(f"  ⏭️  Skipping base schema: {name}")
            skipped_count += 1
            continue
            
        # Skip specific abstract thread types
        if should_skip_schema(name):
            print(f"  ⏭️  Skipping abstract schema: {name}")
            skipped_count += 1
            continue
            
        try:
            with open(schema_file, 'r', encoding='utf-8') as f:
                content = json.load(f)
            
            category = content.get('category', 'unknown')
            
            schemas.append(SchemaInfo(
                name=name,
                category=category,
                file_path=schema_file,
                content=content
            ))
            
        except Exception as e:
            print(f"⚠️  Failed to load {schema_file}: {e}")
            continue
    
    print(f"  📊 Loaded {len(schemas)} schemas, skipped {skipped_count} base schemas")
    return schemas

def categorize_schemas(schemas: List[SchemaInfo]) -> Dict[str, List[SchemaInfo]]:
    """Group schemas by category."""
    categories = {}
    for schema in schemas:
        if schema.category not in categories:
            categories[schema.category] = []
        categories[schema.category].append(schema)
    
    # Sort schemas within each category
    for category in categories:
        categories[category].sort(key=lambda s: s.name)
    
    return categories

def generate_rust_with_typify(schemas: List[SchemaInfo], output_dir: Path) -> None:
    """Generate Rust structs using cargo typify (same pattern as datamodel-code-generator)."""
    
    # Group by category (same as Pydantic)
    categories = categorize_schemas(schemas)
    
    with tempfile.TemporaryDirectory() as temp_dir:
        temp_path = Path(temp_dir)
        
        for category, category_schemas in categories.items():
            print(f"  🦀 Generating {category} structs ({len(category_schemas)} schemas)...")
            
            # Create category directory
            category_dir = output_dir / "src" / category
            category_dir.mkdir(parents=True, exist_ok=True)
            
            # Generate structs for this category
            for schema in category_schemas:
                temp_file = temp_path / f"{schema.name}.json"
                output_file = category_dir / f"{schema.name.lower()}.rs"
                
                try:
                    # Write the schema to temp file (same as Pydantic approach)
                    with open(temp_file, 'w') as f:
                        json.dump(schema.content, f, indent=2)
                    
                    # Build the cargo typify command (mirrors datamodel-codegen pattern)
                    cmd = [
                        "cargo", "typify",
                        str(temp_file),
                        "--output", str(output_file),
                    ]
                    
                    result = subprocess.run(cmd, capture_output=True, text=True, check=True)
                    print(f"    ✅ Generated {schema.name}.rs")
                    
                except subprocess.CalledProcessError as e:
                    print(f"    ❌ Failed to generate {schema.name}: {e.stderr}")
                    continue
            
            # Create mod.rs for category (similar to __init__.py for Pydantic)
            create_category_mod_file(category_dir, category, category_schemas)

def create_category_mod_file(category_dir: Path, category: str, schemas: List[SchemaInfo]) -> None:
    """Create mod.rs file for a category (mirrors Pydantic __init__.py creation)."""
    
    mod_content = f"""//! {category.title()} module
//!
//! Generated Rust structs for {category} schemas.

"""
    
    # Add module declarations
    for schema in schemas:
        module_name = schema.name.lower()
        mod_content += f"pub mod {module_name};\n"
    
    mod_content += "\n// Re-export all types\n"
    
    # Add re-exports (will need to be fixed after generation to get actual type names)
    for schema in schemas:
        module_name = schema.name.lower()
        mod_content += f"pub use {module_name}::*;\n"
    
    mod_file = category_dir / "mod.rs"
    with open(mod_file, 'w') as f:
        f.write(mod_content)
    
    print(f"    📝 Generated {category}/mod.rs")

def create_cargo_project(output_dir: Path, categories: Dict[str, List[SchemaInfo]]) -> None:
    """Create Cargo.toml and lib.rs (mirrors pyproject.toml creation for Pydantic)."""
    
    total_schemas = sum(len(schemas) for schemas in categories.values())
    
    # Create Cargo.toml
    cargo_content = f"""[package]
name = "familiar-schemas-rust"
version = "0.1.0"
edition = "2021"
description = "Generated Rust structs for Familiar schema system"
license = "MIT"

# Generated from {total_schemas} schemas across {len(categories)} categories

[dependencies]
serde = {{ version = "1.0", features = ["derive"] }}
serde_json = "1.0"
uuid = {{ version = "1.0", features = ["v4", "serde"], optional = true }}
chrono = {{ version = "0.4", features = ["serde"], optional = true }}

[features]
default = ["uuid", "chrono"]
uuid = ["dep:uuid"]
chrono = ["dep:chrono"]
"""
    
    cargo_file = output_dir / "Cargo.toml"
    with open(cargo_file, 'w') as f:
        f.write(cargo_content)
    
    # Create src/lib.rs
    src_dir = output_dir / "src"
    src_dir.mkdir(exist_ok=True)
    
    lib_content = f"""//! Familiar Schema System - Rust Structs
//!
//! Generated Rust structs for the Familiar schema system.
//! Generated from {total_schemas} schemas across {len(categories)} categories.

"""
    
    # Add category modules
    for category in sorted(categories.keys()):
        lib_content += f"pub mod {category};\n"
    
    lib_file = src_dir / "lib.rs"
    with open(lib_file, 'w') as f:
        f.write(lib_content)
    
    print(f"📦 Generated Cargo.toml and src/lib.rs")

def main():
    """Main entry point - mirrors Pydantic generator structure."""
    parser = argparse.ArgumentParser(
        description="Generate Rust structs using typify (mirrors Pydantic approach)"
    )
    parser.add_argument(
        "--assembled-dir",
        type=Path,
        default=Path("../../../docs/v3/schemas/assembled"),
        help="Directory containing assembled schemas"
    )
    parser.add_argument(
        "--output-dir", 
        type=Path,
        default=Path("../generated/rust-typify"),
        help="Output directory for generated Rust code"
    )
    
    args = parser.parse_args()
    
    # Resolve paths relative to script location
    script_dir = Path(__file__).parent
    assembled_dir = (script_dir / args.assembled_dir).resolve()
    output_dir = (script_dir / args.output_dir).resolve()
    
    print("🦀 Generating Rust structs with typify")
    print(f"📁 Source: {assembled_dir}")
    print(f"📁 Output: {output_dir}")
    print("=" * 60)
    
    # Load schemas (same logic as Pydantic generator)
    print("📖 Loading assembled schemas...")
    schemas = load_assembled_schemas(assembled_dir)
    print(f"✅ Loaded {len(schemas)} schemas")
    
    # Group by category
    categories = categorize_schemas(schemas)
    print(f"📊 Categories: {dict((k, len(v)) for k, v in categories.items())}")
    
    # Create output directory
    output_dir.mkdir(parents=True, exist_ok=True)
    
    # Generate Rust structs using typify
    print("🏗️  Generating Rust structs with typify...")
    generate_rust_with_typify(schemas, output_dir)
    
    # Create Cargo project files
    print("📝 Creating Cargo project...")
    create_cargo_project(output_dir, categories)
    
    print("=" * 60)
    print(f"🎉 Rust generation with typify complete!")
    print(f"📦 Generated {len(schemas)} structs")
    print(f"📁 Output: {output_dir}")
    
    print(f"""
🔧 Next steps:
1. cd {output_dir}
2. cargo check
3. cargo test
4. cargo build --release
""")

if __name__ == "__main__":
    main() 