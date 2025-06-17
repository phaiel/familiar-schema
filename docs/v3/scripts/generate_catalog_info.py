#!/usr/bin/env python3
"""
Generate Backstage catalog-info.yaml from JSON schemas.

This script analyzes all JSON schema files in the schemas directory and generates
individual Backstage catalog components with proper dependency relationships
based on $ref analysis.
"""

import json
import os
import re
import yaml
from pathlib import Path
from typing import Dict, List, Set, Optional
from urllib.parse import urlparse


class SchemaCatalogGenerator:
    def __init__(self, schemas_dir: str = "schemas"):
        self.schemas_dir = Path(schemas_dir)
        self.schemas: Dict[str, Dict] = {}
        self.schema_files: Dict[str, str] = {}  # component_name -> file_path
        self.dependencies: Dict[str, Set[str]] = {}  # component_name -> set of dependencies
        
    def load_all_schemas(self):
        """Load all JSON schema files from the schemas directory."""
        print(f"Loading schemas from {self.schemas_dir}")
        
        for schema_file in self.schemas_dir.rglob("*.json"):
            if schema_file.name.startswith('.'):
                continue
            
            # Skip assembled schemas if we have the source version
            # (prefer source schemas for documentation purposes)
            relative_path = schema_file.relative_to(self.schemas_dir)
            if 'assembled/' in str(relative_path):
                source_path = self.schemas_dir / str(relative_path).replace('assembled/', '')
                if source_path.exists():
                    continue  # Skip assembled version, use source
                
            try:
                with open(schema_file, 'r') as f:
                    schema_data = json.load(f)
                
                component_name = self.file_path_to_component_name(schema_file)
                self.schemas[component_name] = schema_data
                self.schema_files[component_name] = str(schema_file.relative_to(Path(".")))
                
                print(f"  Loaded: {component_name} <- {schema_file.relative_to(self.schemas_dir)}")
                
            except Exception as e:
                print(f"  Error loading {schema_file}: {e}")
    
    def file_path_to_component_name(self, file_path: Path) -> str:
        """Convert a file path to a Backstage component name."""
        # Remove .schema.json extension and convert to kebab-case
        name = file_path.stem
        if name.endswith('.schema'):
            name = name[:-7]  # Remove .schema
        
        # Convert CamelCase to kebab-case
        name = re.sub(r'([a-z0-9])([A-Z])', r'\1-\2', name).lower()
        
        # Add directory context for disambiguation
        relative_path = file_path.relative_to(self.schemas_dir)
        if len(relative_path.parts) > 1:
            directory = relative_path.parts[0]
            if directory != "_base":  # _base is implied as foundational
                name = f"{directory}-{name}"
        
        return name
    
    def extract_ref_dependencies(self, schema: Dict, component_name: str) -> Set[str]:
        """Extract $ref dependencies from a schema and convert to component names."""
        refs = set()
        
        def find_refs(obj):
            if isinstance(obj, dict):
                for key, value in obj.items():
                    if key == "$ref" and isinstance(value, str):
                        refs.add(value)
                    else:
                        find_refs(value)
            elif isinstance(obj, list):
                for item in obj:
                    find_refs(item)
        
        find_refs(schema)
        
        # Convert $ref paths to component names
        dependencies = set()
        for ref in refs:
            dep_component = self.ref_to_component_name(ref, component_name)
            if dep_component and dep_component != component_name:
                dependencies.add(dep_component)
        
        return dependencies
    
    def ref_to_component_name(self, ref: str, current_component: str) -> Optional[str]:
        """Convert a $ref path to a component name."""
        # Strip URL fragment if present
        if '#/' in ref:
            ref = ref.split('#/')[0]
        
        # Handle relative refs like "./BaseMetadata.schema.json" or "../_base/BaseComponent.schema.json"
        if ref.startswith('./') or ref.startswith('../'):
            # Normalize the path and extract filename
            ref_path = Path(ref)
            filename = ref_path.name
            
            # Remove .schema.json extension
            if filename.endswith('.schema.json'):
                schema_name = filename[:-12]  # Remove .schema.json
            elif filename.endswith('.json'):
                schema_name = filename[:-5]  # Remove .json
            else:
                schema_name = filename
            
            # Convert to component name format
            ref_component = re.sub(r'([a-z0-9])([A-Z])', r'\1-\2', schema_name).lower()
            
            # Determine directory context from the path
            parent_dir = ref_path.parent.name if ref_path.parent.name != '.' else None
            
            if parent_dir == '_base':
                # Base schemas: if already starts with 'base-', don't add another prefix
                if ref_component.startswith('base-'):
                    return ref_component
                else:
                    return f"base-{ref_component}"
            elif parent_dir and parent_dir != '.':
                # Other directory schemas get directory prefix
                return f"{parent_dir}-{ref_component}"
            else:
                # Same directory or no directory context
                if current_component.startswith('base-'):
                    return f"base-{ref_component}"
                elif '-' in current_component:
                    current_dir = current_component.split('-')[0]
                    return f"{current_dir}-{ref_component}"
                else:
                    return ref_component
        
        # Handle URL-based refs
        if ref.startswith('http'):
            parsed = urlparse(ref)
            path_parts = parsed.path.strip('/').split('/')
            if path_parts and path_parts[-1].endswith('.schema.json'):
                schema_name = path_parts[-1][:-12]  # Remove .schema.json
                return re.sub(r'([a-z0-9])([A-Z])', r'\1-\2', schema_name).lower()
        
        return None
    
    def analyze_dependencies(self):
        """Analyze all schemas to build dependency graph."""
        print("\nAnalyzing dependencies...")
        
        for component_name, schema in self.schemas.items():
            deps = self.extract_ref_dependencies(schema, component_name)
            self.dependencies[component_name] = deps
            
            if deps:
                print(f"  {component_name} depends on: {', '.join(sorted(deps))}")
            else:
                print(f"  {component_name} has no dependencies")
    
    def get_schema_category(self, file_path: str) -> str:
        """Determine the category/type of a schema based on its path."""
        if '_base/' in file_path:
            return 'foundation'
        elif 'components/' in file_path:
            return 'component'
        elif 'entities/' in file_path:
            return 'entity'
        elif 'snippets/' in file_path:
            return 'snippet'
        elif 'templates/' in file_path:
            return 'template'
        elif 'api/' in file_path:
            return 'api'
        elif 'events/' in file_path:
            return 'event'
        elif 'laws/' in file_path:
            return 'law'
        else:
            return 'schema'
    
    def get_tags_for_schema(self, schema: Dict, file_path: str) -> List[str]:
        """Generate appropriate tags for a schema."""
        tags = ['schema', 'auto-generated']
        
        # Add category-based tags
        category = self.get_schema_category(file_path)
        tags.append(category)
        
        # Add tags based on schema content
        title = schema.get('title', '').lower()
        description = schema.get('description', '').lower()
        
        if any(word in title + description for word in ['physics', 'quantum', 'energy']):
            tags.append('physics')
        if any(word in title + description for word in ['cognitive', 'memory', 'perspective']):
            tags.append('cognitive')
        if any(word in title + description for word in ['tenant', 'user', 'member']):
            tags.append('tenant')
        if any(word in title + description for word in ['component', 'ecs']):
            tags.append('ecs')
        if any(word in title + description for word in ['thread', 'entity']):
            tags.append('entity')
        if any(word in title + description for word in ['base', 'foundation', 'shared']):
            tags.append('foundation')
        
        return sorted(list(set(tags)))
    
    def get_subcomponent_relationships(self, component_name: str, schema: Dict) -> Optional[str]:
        """Determine if this schema should be a subcomponent of another."""
        file_path = self.schema_files[component_name]
        
        # Rule 1: Non-base schemas that extend base schemas should be subcomponents
        if not component_name.startswith('base-'):
            deps = self.dependencies.get(component_name, set())
            
            # Find the most specific base dependency, but prioritize direct inheritance
            base_deps = [dep for dep in deps if dep.startswith('base-')]
            if base_deps:
                # For components, prefer base-component if present
                if 'components/' in file_path and 'base-component' in base_deps:
                    return 'component:default/base-component'
                # For entities, prefer base-entity or more specific entity bases
                elif 'entities/' in file_path:
                    entity_bases = [dep for dep in base_deps if 'entity' in dep]
                    if entity_bases:
                        # Sort by specificity (longer names are more specific)
                        entity_bases.sort(key=len, reverse=True)
                        return f'component:default/{entity_bases[0]}'
                # Otherwise, use the most specific base
                base_deps.sort(key=len, reverse=True)
                return f'component:default/{base_deps[0]}'
        
        # Rule 2: Entity schemas that inherit from other entity schemas
        if 'entities/' in file_path and not component_name.startswith('base-'):
            # Check for inheritance patterns in allOf
            all_of = schema.get('allOf', [])
            for item in all_of:
                if '$ref' in item:
                    ref_comp = self.ref_to_component_name(item['$ref'], component_name)
                    if ref_comp and ref_comp != component_name and ref_comp in self.schemas:
                        return f'component:default/{ref_comp}'
        
        # Rule 3: Component schemas that extend BaseComponent
        if 'components/' in file_path and not component_name.startswith('base-'):
            deps = self.dependencies.get(component_name, set())
            if 'base-component' in deps:
                return 'component:default/base-component'
        
        return None

    def generate_component_entry(self, component_name: str) -> Dict:
        """Generate a Backstage component entry for a schema."""
        schema = self.schemas[component_name]
        file_path = self.schema_files[component_name]
        
        # Extract metadata from schema
        title = schema.get('title', component_name.replace('-', ' ').title())
        description = schema.get('description', f'JSON schema: {title}')
        schema_id = schema.get('$id', '')
        
        # Build component entry
        entry = {
            'apiVersion': 'backstage.io/v1alpha1',
            'kind': 'Component',
            'metadata': {
                'name': component_name,
                'title': title,
                'description': description,
                'tags': self.get_tags_for_schema(schema, file_path),
                'annotations': {
                    'github.com/project-slug': 'phaiel/familiar-schema',
                    'backstage.io/source-location': f'url:https://github.com/phaiel/familiar-schema/blob/main/{file_path}',
                    'familiar.dev/schema-file': file_path
                }
            },
            'spec': {
                'type': 'library',
                'lifecycle': 'production',
                'owner': 'user:default/phaiel',
                'system': 'familiar-schema-system'
            }
        }
        
        # Add schema ID annotation if present
        if schema_id:
            entry['metadata']['annotations']['familiar.dev/schema-id'] = schema_id
        
        # Add subcomponentOf relationship if applicable
        subcomponent_of = self.get_subcomponent_relationships(component_name, schema)
        if subcomponent_of:
            entry['spec']['subcomponentOf'] = subcomponent_of
        
        # Add dependencies if any
        deps = self.dependencies.get(component_name, set())
        if deps:
            # Convert to proper component references
            dep_refs = [f'component:default/{dep}' for dep in sorted(deps) 
                       if dep in self.schemas]  # Only include deps that exist
            if dep_refs:
                entry['spec']['dependsOn'] = dep_refs
        
        return entry
    
    def generate_catalog_info(self) -> Dict:
        """Generate the complete catalog-info.yaml structure."""
        print("\nGenerating catalog entries...")
        
        # Start with existing system-level components
        catalog_entries = [
            # System
            {
                'apiVersion': 'backstage.io/v1alpha1',
                'kind': 'System',
                'metadata': {
                    'name': 'familiar-schema-system',
                    'title': 'Familiar Schema System',
                    'description': 'Sophisticated JSON schema system with physics simulation, cognitive modeling, ECS components, assembly pipeline, and Rust code generation',
                    'tags': ['schema', 'system', 'json-schema', 'rust', 'code-generation', 'physics', 'cognitive', 'ecs', 'quantum', 'assembly'],
                    'annotations': {
                        'github.com/project-slug': 'phaiel/familiar-schema',
                        'backstage.io/source-location': 'url:https://github.com/phaiel/familiar-schema/tree/main/',
                        'backstage.io/techdocs-ref': 'dir:.'
                    }
                },
                'spec': {
                    'owner': 'user:default/phaiel',
                    'domain': 'platform'
                }
            },
            
            # Assembly Pipeline
            {
                'apiVersion': 'backstage.io/v1alpha1',
                'kind': 'Component',
                'metadata': {
                    'name': 'schema-assembly-pipeline',
                    'title': 'Schema Assembly Pipeline',
                    'description': 'JSON schema assembly and code generation pipeline using Make, quicktype, and bundling tools',
                    'tags': ['schema', 'pipeline', 'code-generation', 'json-schema', 'makefile', 'quicktype'],
                    'annotations': {
                        'github.com/project-slug': 'phaiel/familiar-schema',
                        'backstage.io/source-location': 'url:https://github.com/phaiel/familiar-schema/tree/main/docs/v3/'
                    }
                },
                'spec': {
                    'type': 'library',
                    'lifecycle': 'production',
                    'owner': 'user:default/phaiel',
                    'system': 'familiar-schema-system'
                }
            },
            
            # Generated Rust Types
            {
                'apiVersion': 'backstage.io/v1alpha1',
                'kind': 'Component',
                'metadata': {
                    'name': 'generated-rust-types',
                    'title': 'Generated Rust Types',
                    'description': 'Auto-generated Rust data structures from JSON schemas using quicktype',
                    'tags': ['rust', 'generated', 'types', 'code-generation', 'quicktype'],
                    'annotations': {
                        'github.com/project-slug': 'phaiel/familiar-schema',
                        'backstage.io/source-location': 'url:https://github.com/phaiel/familiar-schema/tree/main/src/generated/'
                    }
                },
                'spec': {
                    'type': 'library',
                    'lifecycle': 'production',
                    'owner': 'user:default/phaiel',
                    'system': 'familiar-schema-system',
                    'dependsOn': ['component:default/schema-assembly-pipeline']
                }
            }
        ]
        
        # Add individual schema components
        for component_name in sorted(self.schemas.keys()):
            entry = self.generate_component_entry(component_name)
            catalog_entries.append(entry)
            print(f"  Generated: {component_name}")
        
        # Add user and domain
        catalog_entries.extend([
            {
                'apiVersion': 'backstage.io/v1alpha1',
                'kind': 'User',
                'metadata': {
                    'name': 'phaiel',
                    'title': 'Schema System Owner'
                },
                'spec': {
                    'profile': {'displayName': 'phaiel'},
                    'memberOf': []
                }
            },
            {
                'apiVersion': 'backstage.io/v1alpha1',
                'kind': 'Domain',
                'metadata': {
                    'name': 'platform',
                    'title': 'Platform Domain',
                    'description': 'Platform infrastructure and development tools'
                },
                'spec': {
                    'owner': 'user:default/phaiel'
                }
            }
        ])
        
        return catalog_entries
    
    def write_catalog_info(self, output_file: str = "catalog-info.yaml"):
        """Write the generated catalog to a YAML file."""
        catalog_entries = self.generate_catalog_info()
        
        print(f"\nWriting catalog to {output_file}")
        print(f"Generated {len(catalog_entries)} catalog entries")
        
        with open(output_file, 'w') as f:
            for i, entry in enumerate(catalog_entries):
                if i > 0:
                    f.write('---\n')
                yaml.dump(entry, f, default_flow_style=False, sort_keys=False)
        
        print(f"Catalog successfully written to {output_file}")
    
    def run(self):
        """Run the complete catalog generation process."""
        print("=== Familiar Schema Catalog Generator ===")
        self.load_all_schemas()
        self.analyze_dependencies()
        self.write_catalog_info()
        print("\n=== Generation Complete ===")


if __name__ == "__main__":
    generator = SchemaCatalogGenerator()
    generator.run() 