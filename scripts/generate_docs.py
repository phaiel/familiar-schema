#!/usr/bin/env python3
"""
Familiar Schema Documentation Generator

Automatically generates comprehensive TechDocs documentation from JSON schemas.
Creates structured Markdown files for MkDocs/Backstage TechDocs integration.
"""

import json
import yaml
from pathlib import Path
from typing import Dict, List, Set, Optional, Any
import re
from datetime import datetime


class SchemaDocGenerator:
    """Generates comprehensive documentation from JSON schemas."""
    
    def __init__(self, schemas_dir: str = "schemas", docs_dir: str = "../docs"):
        self.schemas_dir = Path(schemas_dir)
        self.docs_dir = Path(docs_dir)
        self.schemas: Dict[str, Dict] = {}
        self.schema_files: Dict[str, Path] = {}
        self.dependencies: Dict[str, Set[str]] = {}
        
    def load_all_schemas(self):
        """Load all JSON schema files."""
        print("Loading schemas...")
        
        for schema_file in self.schemas_dir.rglob("*.json"):
            try:
                with open(schema_file, 'r') as f:
                    schema = json.load(f)
                
                component_name = self.file_path_to_component_name(schema_file)
                self.schemas[component_name] = schema
                self.schema_files[component_name] = schema_file.relative_to(Path.cwd())
                print(f"  Loaded: {component_name}")
                
            except Exception as e:
                print(f"  Error loading {schema_file}: {e}")
        
        print(f"Loaded {len(self.schemas)} schemas")
    
    def file_path_to_component_name(self, file_path: Path) -> str:
        """Convert file path to component name."""
        rel_path = file_path.relative_to(self.schemas_dir)
        
        # Remove .schema.json or .json extension
        name = rel_path.stem
        if name.endswith('.schema'):
            name = name[:-7]
        
        # Convert directory structure to component name
        parts = list(rel_path.parts[:-1])  # All parts except filename
        
        # Handle special directories
        if parts and parts[0] == '_base':
            parts = ['base']
        elif parts and parts[0] == 'snippets':
            # For snippets, include the full path structure
            parts = ['snippets'] + parts[1:]
        
        # Add the schema name
        parts.append(name)
        
        # Convert to kebab-case component name
        component_name = '-'.join(parts)
        component_name = re.sub(r'([a-z0-9])([A-Z])', r'\1-\2', component_name).lower()
        
        return component_name
    
    def generate_simple_docs(self):
        """Generate a simple documentation structure."""
        print("\nGenerating documentation...")
        
        # Create docs directory structure
        docs_root = self.docs_dir / 'docs'
        docs_root.mkdir(exist_ok=True)
        
        # Generate main index
        home_content = f"""# Familiar Schema System

Auto-generated documentation for the Familiar Schema System.

*Generated on {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}*

## Overview

The Familiar Schema System contains **{len(self.schemas)} schemas** organized in a hierarchical structure:

## Schema Catalog

"""
        
        # Group schemas by category
        categories = {}
        for component_name in self.schemas.keys():
            file_path = self.schema_files[component_name]
            category = self.get_schema_category(file_path)
            if category not in categories:
                categories[category] = []
            categories[category].append(component_name)
        
        for category, schemas in sorted(categories.items()):
            home_content += f"\n### {category.title()} Schemas ({len(schemas)})\n\n"
            for schema_name in sorted(schemas):
                schema = self.schemas[schema_name]
                title = schema.get('title', schema_name.replace('-', ' ').title())
                description = schema.get('description', 'No description')
                home_content += f"- **{title}**: {description}\n"
        
        with open(docs_root / 'index.md', 'w') as f:
            f.write(home_content)
        
        # Generate simple MkDocs config
        config = {
            'site_name': 'Familiar Schema System',
            'site_description': 'Auto-generated schema documentation',
            'nav': [
                {'Home': 'index.md'}
            ],
            'plugins': ['techdocs-core'],
            'theme': {
                'name': 'material',
                'palette': {
                    'primary': 'blue'
                }
            }
        }
        
        with open(self.docs_dir / 'mkdocs.yml', 'w') as f:
            yaml.dump(config, f, default_flow_style=False)
        
        print(f"Generated documentation in {docs_root}")
    
    def get_schema_category(self, file_path: Path) -> str:
        """Determine schema category from file path."""
        parts = file_path.parts
        if '_base' in parts:
            return 'foundation'
        elif 'components' in parts:
            return 'component'
        elif 'entities' in parts:
            return 'entity'
        elif 'snippets' in parts:
            return 'snippet'
        elif 'assembled' in parts:
            return 'assembled'
        else:
            return 'other'
    
    def run(self):
        """Run the documentation generation process."""
        print("=== Familiar Schema Documentation Generator ===")
        self.load_all_schemas()
        self.generate_simple_docs()
        print("\n=== Documentation Generation Complete ===")


if __name__ == "__main__":
    generator = SchemaDocGenerator()
    generator.run() 