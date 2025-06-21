#!/usr/bin/env python3
"""
Familiar Schema Pydantic Model Generator

Production-quality code generator for Pydantic models from JSON schemas.
Uses datamodel-code-generator for reliable model generation.

Features:
- Robust error handling and validation
- Clear logging and progress reporting  
- Proper file organization by schema category
- Skip base/abstract schemas automatically
- Generate package structure with proper imports
"""

import json
import argparse
import shutil
import sys
import subprocess
import tempfile
from pathlib import Path
from typing import Dict, Any, List, Optional
from dataclasses import dataclass

@dataclass
class SchemaInfo:
    """Schema metadata for generation."""
    name: str
    category: str
    file_path: Path
    content: Dict[str, Any]

class PydanticModelGenerator:
    """Production Pydantic model generator."""
    
    def __init__(self, assembled_dir: Path, output_dir: Path):
        self.assembled_dir = assembled_dir
        self.output_dir = output_dir
        self.schemas: List[SchemaInfo] = []
        
    def validate_environment(self) -> bool:
        """Validate that required tools are available."""
        try:
            result = subprocess.run(
                ["datamodel-codegen", "--version"], 
                capture_output=True, 
                check=True,
                text=True
            )
            print(f"✅ datamodel-code-generator: {result.stdout.strip()}")
            return True
        except (subprocess.CalledProcessError, FileNotFoundError):
            print("❌ datamodel-code-generator not found!")
            print("   Install with: pip install datamodel-code-generator")
            return False
    
    def should_skip_schema(self, name: str) -> bool:
        """Determine if schema should be skipped (base/abstract schemas)."""
        return name.startswith("Base")
    
    def load_schemas(self) -> bool:
        """Load all assembled schemas, excluding base schemas."""
        if not self.assembled_dir.exists():
            print(f"❌ Schema directory not found: {self.assembled_dir}")
            return False
            
        print(f"📂 Loading schemas from: {self.assembled_dir}")
        
        schemas = []
        skipped = 0
        
        for schema_file in self.assembled_dir.glob("*.schema.json"):
            if schema_file.name == "assembly_manifest.json":
                continue
                
            name = schema_file.stem.replace('.schema', '')
            
            if self.should_skip_schema(name):
                print(f"  ⏭️  Skipping base schema: {name}")
                skipped += 1
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
        
        self.schemas = schemas
        print(f"  📊 Loaded {len(schemas)} schemas, skipped {skipped} base schemas")
        return len(schemas) > 0
    
    def categorize_schemas(self) -> Dict[str, List[SchemaInfo]]:
        """Group schemas by category."""
        categories = {}
        for schema in self.schemas:
            if schema.category not in categories:
                categories[schema.category] = []
            categories[schema.category].append(schema)
        
        # Sort within categories
        for category_schemas in categories.values():
            category_schemas.sort(key=lambda s: s.name)
            
        return categories
    
    def generate_model(self, schema: SchemaInfo, output_file: Path) -> bool:
        """Generate a single Pydantic model using datamodel-codegen."""
        with tempfile.NamedTemporaryFile(mode='w', suffix='.json', delete=False) as temp_file:
            json.dump(schema.content, temp_file, indent=2)
            temp_path = temp_file.name
        
        try:
            cmd = [
                "datamodel-codegen",
                "--input", temp_path,
                "--input-file-type", "jsonschema", 
                "--output", str(output_file),
                "--target-python-version", "3.9",
                "--use-schema-description",
                "--reuse-model",
                "--enable-faux-immutability",
                "--class-name", schema.name,
                "--output-model-type", "pydantic_v2.BaseModel",
                "--use-union-operator",
                "--use-generic-container-types",
                "--use-annotated",
                "--disable-appending-item-suffix",
                "--use-subclass-enum",
                "--strip-default-none"
            ]
            
            subprocess.run(cmd, capture_output=True, check=True, text=True)
            
            # Post-process to fix required nullable fields if needed
            self._fix_required_nullable_fields(output_file, schema)
            
            return True
            
        except subprocess.CalledProcessError as e:
            print(f"    ❌ Failed to generate {schema.name}: {e.stderr}")
            return False
        finally:
            Path(temp_path).unlink(missing_ok=True)
    
    def _fix_required_nullable_fields(self, output_file: Path, schema: SchemaInfo):
        """Fix required nullable fields that datamodel-codegen handles incorrectly."""
        import re
        
        if not output_file.exists():
            return
            
        content = output_file.read_text()
        
        # Get required fields from schema
        required_fields = set(schema.content.get("required", []))
        properties = schema.content.get("properties", {})
        
        # Find required fields that are nullable (have null in type array)
        required_nullable = []
        for field_name in required_fields:
            if field_name in properties:
                prop = properties[field_name]
                field_type = prop.get("type", [])
                if isinstance(field_type, list) and "null" in field_type:
                    required_nullable.append(field_name)
        
        # Fix these fields in the generated Python code
        modified = False
        for field_name in required_nullable:
            # Handle both regular and Annotated field patterns:
            # Pattern 1: field_name: Type | None = None
            # Pattern 2: field_name: Annotated[...] = None (multi-line)
            
            # First try simple pattern
            pattern1 = rf'(\s+{field_name}:\s+[^=\n]+)\s*=\s*None'
            if re.search(pattern1, content):
                content = re.sub(pattern1, r'\1', content)
                modified = True
            else:
                # Try multi-line Annotated pattern
                # Look for field_name: Annotated[...] = None across multiple lines
                pattern2 = rf'(\s+{field_name}:\s+Annotated\[[\s\S]*?\]\s*)\s*=\s*None'
                if re.search(pattern2, content):
                    content = re.sub(pattern2, r'\1', content)
                    modified = True
        
        # CRITICAL FIX: Rename main class to exact schema name for contract validator
        content = self._fix_main_class_name(content, schema)
        
        if modified or required_nullable:
            output_file.write_text(content)
            if required_nullable:
                print(f"    🔧 Fixed required nullable fields in {schema.name}: {required_nullable}")
    
    def _fix_main_class_name(self, content: str, schema: SchemaInfo) -> str:
        """Rename the main model class to exactly match schema name for contract validator."""
        import re
        
        # For schemas like Thread_state_log, we need the class to be named exactly Thread_state_log
        # not ThreadStateLog, so the contract validator finds it with score 100
        
        expected_class_name = schema.name
        
        # Find the main class (usually the longest/most complex one)
        class_pattern = r'class\s+(\w+)\(BaseModel\):'
        classes = re.findall(class_pattern, content)
        
        if not classes:
            return content
        
        # Find the main class (heuristic: the one with the most properties or the title-derived name)
        main_class = None
        
        # Look for classes that might be the main one
        for class_name in classes:
            # Skip obviously helper classes
            if class_name in ['Columns', 'Metadata', 'State', 'Reason', 'Status']:
                continue
            # This is likely the main class
            main_class = class_name
            break
        
        if main_class and main_class != expected_class_name:
            # Replace the class name
            old_pattern = rf'class\s+{main_class}\(BaseModel\):'
            new_replacement = f'class {expected_class_name}(BaseModel):'
            
            if re.search(old_pattern, content):
                content = re.sub(old_pattern, new_replacement, content)
                print(f"    🏷️  Renamed main class {main_class} → {expected_class_name}")
        
        return content
    
    def create_category_init(self, category_dir: Path, category: str, schemas: List[SchemaInfo]) -> None:
        """Create __init__.py for a category directory."""
        content_lines = [
            f'"""',
            f'{category.title()} module',
            f'',
            f'Generated Pydantic models for {category} schemas.',
            f'"""',
            f'',
        ]
        
        # Add imports
        for schema in schemas:
            content_lines.append(f"from .{schema.name} import *")
        
        content_lines.extend([
            "",
            f"# Category registry", 
            f"{category.upper()}_MODELS = [",
            f"    # Models will be populated by import repair script",
            f"]",
            ""
        ])
        
        init_file = category_dir / "__init__.py"
        with open(init_file, 'w', encoding='utf-8') as f:
            f.write('\n'.join(content_lines))
        
        print(f"    📝 Created {category}/__init__.py")
    
    def create_main_init(self, categories: Dict[str, List[SchemaInfo]]) -> None:
        """Create main package __init__.py."""
        total_schemas = sum(len(schemas) for schemas in categories.values())
        
        content_lines = [
            '"""',
            'Comprehensive Familiar Schema Library',
            '',
            'Auto-generated Pydantic models for all Familiar schemas including:',
        ]
        
        for category, schemas in sorted(categories.items()):
            content_lines.append(f"- {category}: {len(schemas)} models")
        
        content_lines.extend([
            f"",
            f"Total: {total_schemas} models",
            f"",
            f"Note: Imports and registries are managed by the repair script",
            f"to ensure correct class names are detected from generated files.",
            f'"""',
            f"",
            f"# Version info", 
            f"__version__ = '1.0.0'",
            f"",
            f"# Placeholder registries",
            f"ALL_MODELS = {{}}",
        ])
        
        for category in sorted(categories.keys()):
            content_lines.append(f"{category.upper()}_MODELS = []")
        
        content_lines.append("")
        
        init_file = self.output_dir / "__init__.py"
        with open(init_file, 'w', encoding='utf-8') as f:
            f.write('\n'.join(content_lines))
        
        print(f"📝 Created main __init__.py")
    
    def create_pyproject_toml(self, categories: Dict[str, List[SchemaInfo]]) -> None:
        """Create pyproject.toml for the package."""
        total_schemas = sum(len(schemas) for schemas in categories.values())
        
        content = f"""[build-system]
requires = ["setuptools>=61.0", "wheel"]
build-backend = "setuptools.build_meta"

[project]
name = "familiar-schemas"
version = "1.0.0"
description = "Comprehensive Pydantic models for Familiar schema system"
readme = "README.md"
requires-python = ">=3.9"
license = {{text = "MIT"}}
authors = [
    {{name = "Familiar Team"}}
]
classifiers = [
    "Development Status :: 5 - Production/Stable",
    "Intended Audience :: Developers", 
    "License :: OSI Approved :: MIT License",
    "Programming Language :: Python :: 3",
    "Programming Language :: Python :: 3.9",
    "Programming Language :: Python :: 3.10",
    "Programming Language :: Python :: 3.11",
    "Programming Language :: Python :: 3.12",
]

dependencies = [
    "pydantic>=2.0.0",
    "typing-extensions>=4.0.0",
]

[project.optional-dependencies]
dev = [
    "pytest>=7.0.0",
    "pytest-cov>=4.0.0",
    "black>=23.0.0",
    "isort>=5.0.0",
    "mypy>=1.0.0",
]

# Generated from {total_schemas} schemas across {len(categories)} categories
"""
        
        pyproject_file = self.output_dir / "pyproject.toml"
        with open(pyproject_file, 'w', encoding='utf-8') as f:
            f.write(content)
        
        print(f"📝 Created pyproject.toml")
    
    def generate_all_models(self, clean: bool = False) -> bool:
        """Generate all Pydantic models."""
        if clean and self.output_dir.exists():
            print(f"🧹 Cleaning output directory: {self.output_dir}")
            shutil.rmtree(self.output_dir)
        
        self.output_dir.mkdir(parents=True, exist_ok=True)
        
        categories = self.categorize_schemas()
        total_generated = 0
        total_failed = 0
        
        print(f"\n🏗️  Generating Pydantic models...")
        
        for category, category_schemas in categories.items():
            print(f"  🐍 Generating {category} models ({len(category_schemas)} schemas)...")
            
            category_dir = self.output_dir / category
            category_dir.mkdir(parents=True, exist_ok=True)
            
            category_generated = 0
            
            for schema in category_schemas:
                output_file = category_dir / f"{schema.name}.py"
                
                if self.generate_model(schema, output_file):
                    print(f"    ✅ Generated {schema.name}.py")
                    category_generated += 1
                else:
                    total_failed += 1
            
            total_generated += category_generated
            self.create_category_init(category_dir, category, category_schemas)
            print(f"    📊 {category}: {category_generated}/{len(category_schemas)} success")
        
        # Create package files
        self.create_main_init(categories)
        self.create_pyproject_toml(categories)
        
        # Print summary
        total_schemas = total_generated + total_failed
        success_rate = (total_generated / total_schemas * 100) if total_schemas > 0 else 0
        
        print(f"\n🎯 Generation Summary:")
        print(f"   ✅ Generated: {total_generated}")
        print(f"   ❌ Failed: {total_failed}")
        print(f"   📊 Success Rate: {success_rate:.1f}% ({total_generated}/{total_schemas})")
        
        return total_failed == 0

def main() -> int:
    """Main entry point."""
    parser = argparse.ArgumentParser(
        description="Generate comprehensive Pydantic models from assembled schemas",
        formatter_class=argparse.RawDescriptionHelpFormatter
    )
    parser.add_argument(
        "--assembled-dir",
        type=Path,
        default=Path("schemas/assembled"),
        help="Directory containing assembled schemas"
    )
    parser.add_argument(
        "--output-dir", 
        type=Path,
        default=Path("familiar_schemas"),
        help="Output directory for generated models"
    )
    parser.add_argument(
        "--clean",
        action="store_true",
        help="Clean output directory before generation"
    )
    
    args = parser.parse_args()
    
    print("🚀 Familiar Schema → Pydantic Model Generator")
    print("=" * 50)
    
    # Initialize generator
    generator = PydanticModelGenerator(args.assembled_dir, args.output_dir)
    
    # Validate environment
    if not generator.validate_environment():
        return 1
    
    # Load schemas
    if not generator.load_schemas():
        print("❌ No schemas found to generate")
        return 1
    
    # Generate models
    success = generator.generate_all_models(args.clean)
    
    if success:
        print(f"\n✨ Generation complete! Models written to: {args.output_dir}")
        print(f"   Next steps:")
        print(f"   1. Test models: python tests/test_contracts.py")
        return 0
    else:
        print(f"\n❌ Generation completed with errors")
        return 1

if __name__ == "__main__":
    sys.exit(main()) 