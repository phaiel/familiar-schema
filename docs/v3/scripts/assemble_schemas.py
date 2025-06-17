#!/usr/bin/env python3
"""
Schema Assembly Script for Familiar v3

This script assembles final schemas from Jinja2 templates and JSON snippets.
It reads an assembly manifest that defines which templates to use and which
snippets to include for each schema.
"""

import os
import json
import shutil
from pathlib import Path
from jinja2 import Environment, FileSystemLoader, Template

def load_json_file(file_path):
    """Load and parse a JSON file."""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            return json.load(f)
    except Exception as e:
        print(f"Error loading {file_path}: {e}")
        return None

def load_snippets(snippets_dir):
    """Load all field and type snippets."""
    snippets = {}
    
    # Load field snippets
    fields_dir = snippets_dir / 'fields'
    if fields_dir.exists():
        for snippet_file in fields_dir.glob('*.json'):
            snippet_path = f"fields/{snippet_file.name}"
            snippet_data = load_json_file(snippet_file)
            if snippet_data:
                snippets[snippet_path] = snippet_data
    
    # Load type snippets (including subdirectories)
    types_dir = snippets_dir / 'types'
    if types_dir.exists():
        for snippet_file in types_dir.rglob('*.json'):
            # Get relative path from types directory
            rel_path = snippet_file.relative_to(types_dir)
            snippet_path = f"types/{rel_path}"
            snippet_data = load_json_file(snippet_file)
            if snippet_data:
                snippets[snippet_path] = snippet_data
    
    return snippets

def resolve_snippet_references(obj, snippets, snippets_dir):
    """Recursively resolve $snippet references in an object."""
    if isinstance(obj, dict):
        if '$snippet' in obj:
            # This is a snippet reference
            snippet_path = obj['$snippet']
            if snippet_path in snippets:
                return snippets[snippet_path]
            else:
                # Snippet not found, try to provide a fallback
                print(f"⚠️  Warning: Snippet '{snippet_path}' not found, using fallback")
                return {"type": "object", "description": f"Missing snippet: {snippet_path}"}
        else:
            # Regular object, recursively resolve
            return {k: resolve_snippet_references(v, snippets, snippets_dir) for k, v in obj.items()}
    elif isinstance(obj, list):
        return [resolve_snippet_references(item, snippets, snippets_dir) for item in obj]
    else:
        return obj

def resolve_ref_references(obj, snippets, schemas_dir, visited=None):
    """Recursively resolve $ref references that point to snippet files."""
    if visited is None:
        visited = set()
    
    if isinstance(obj, dict):
        if '$ref' in obj:
            # This is a $ref reference
            ref_path = obj['$ref']
            
            # Ensure ref_path is a string
            if not isinstance(ref_path, str):
                print(f"⚠️  Warning: $ref value is not a string: {ref_path}")
                return obj
            
            # Check if this points to a snippet file (only process actual snippet references)
            snippet_key = None
            
            if '../snippets/' in ref_path:
                # Convert relative path to snippet key
                snippet_key = ref_path.replace('../snippets/', '')
            elif ref_path.startswith('../') and any(snip_dir in ref_path for snip_dir in ['workflow/', 'database/', 'primitives/', 'physics/', 'validation/']):
                # Handle references within snippets like "../workflow/TaskDefinition.json"
                snippet_key = ref_path.replace('../', '')
                if not snippet_key.startswith('types/'):
                    snippet_key = f"types/{snippet_key}"
            
            # If this is not a snippet reference, leave the $ref as-is
            if snippet_key is None:
                return obj
                
            # Avoid infinite recursion
            if snippet_key in visited:
                print(f"⚠️  Warning: Circular reference detected for '{snippet_key}'")
                return {"type": "object", "description": f"Circular reference: {snippet_key}"}
            
            if snippet_key in snippets:
                visited.add(snippet_key)
                # Return the snippet content (without the $id to avoid conflicts)
                snippet_content = snippets[snippet_key].copy()
                if '$id' in snippet_content:
                    snippet_content.pop('$id')
                
                # Recursively resolve references within the snippet content
                resolved_content = resolve_ref_references(snippet_content, snippets, schemas_dir, visited)
                visited.remove(snippet_key)
                return resolved_content
            else:
                print(f"⚠️  Warning: Snippet reference '{ref_path}' -> '{snippet_key}' not found")
                return {"type": "object", "description": f"Missing snippet: {snippet_key}"}
            
            # For other $ref references (like BaseTypeSystem.schema.json), leave as-is
            return obj
        else:
            # Regular object, recursively resolve
            return {k: resolve_ref_references(v, snippets, schemas_dir, visited) for k, v in obj.items()}
    elif isinstance(obj, list):
        return [resolve_ref_references(item, snippets, schemas_dir, visited) for item in obj]
    else:
        return obj

def copy_base_schemas(schemas_dir, assembled_dir, snippets):
    """Copy and process base schemas to assembled directory."""
    base_dir = schemas_dir / '_base'
    assembled_base_dir = assembled_dir / '_base'
    
    # Ensure the assembled base directory exists
    assembled_base_dir.mkdir(parents=True, exist_ok=True)
    
    # Copy all base schema files, resolving snippet references
    if base_dir.exists():
        for schema_file in base_dir.glob('*.schema.json'):
            # Load the schema
            schema_data = load_json_file(schema_file)
            if schema_data:
                # Resolve $ref references to snippets
                resolved_schema = resolve_ref_references(schema_data, snippets, schemas_dir)
                
                # Write the processed schema
                dest_file = assembled_base_dir / schema_file.name
                with open(dest_file, 'w', encoding='utf-8') as f:
                    json.dump(resolved_schema, f, indent=2, ensure_ascii=False)
                print(f"✓ Copied base schema {schema_file.name}")
            else:
                # If loading failed, just copy the file as-is
                dest_file = assembled_base_dir / schema_file.name
                shutil.copy2(schema_file, dest_file)
                print(f"✓ Copied base schema {schema_file.name} (no processing)")

def resolve_schema_references(obj, assembled_dir):
    """Recursively resolve $ref references to assembled schemas by inlining their content."""
    if isinstance(obj, dict):
        if '$ref' in obj:
            ref_path = obj['$ref']
            
            # Only process schema references (not snippet references)
            if isinstance(ref_path, str) and ref_path.endswith('.schema.json'):
                schema_file = None
                
                # Handle different reference patterns
                if ref_path.startswith('../'):
                    # Remove the '../' and construct path within assembled directory
                    clean_path = ref_path.replace('../', '')
                    schema_file = assembled_dir / clean_path
                elif ref_path.startswith('./'):
                    # Convert ./ to ../_base/ (most component references are to base schemas)
                    clean_path = ref_path.replace('./', '_base/')
                    schema_file = assembled_dir / clean_path
                    
                if schema_file and schema_file.exists():
                    # Load and inline the referenced schema
                    referenced_schema = load_json_file(schema_file)
                    if referenced_schema:
                        # Remove $id and $schema to avoid conflicts when inlining
                        if '$id' in referenced_schema:
                            referenced_schema.pop('$id')
                        if '$schema' in referenced_schema:
                            referenced_schema.pop('$schema')
                        
                        # Recursively resolve references in the inlined schema
                        resolved_schema = resolve_schema_references(referenced_schema, assembled_dir)
                        return resolved_schema
                    else:
                        print(f"⚠️  Warning: Could not load referenced schema: {schema_file}")
                        return obj
                elif schema_file:
                    print(f"⚠️  Warning: Referenced schema not found: {schema_file}")
                    return obj
            
            # For non-schema references, return as-is
            return obj
        else:
            # Regular object, recursively resolve
            return {k: resolve_schema_references(v, assembled_dir) for k, v in obj.items()}
    elif isinstance(obj, list):
        return [resolve_schema_references(item, assembled_dir) for item in obj]
    else:
        return obj

def copy_entity_schemas(schemas_dir, assembled_dir, snippets):
    """Copy and process entity schemas to assembled directory with full dereferencing."""
    entities_dir = schemas_dir / 'entities'
    assembled_entities_dir = assembled_dir / 'entities'
    
    # Ensure the assembled entities directory exists
    assembled_entities_dir.mkdir(parents=True, exist_ok=True)
    
    # Copy all entity schema files, resolving all references
    if entities_dir.exists():
        for schema_file in entities_dir.glob('*.schema.json'):
            # Load the schema
            schema_data = load_json_file(schema_file)
            if schema_data:
                # First resolve snippet references
                resolved_schema = resolve_ref_references(schema_data, snippets, schemas_dir)
                
                # Then resolve schema references by inlining assembled schemas
                fully_resolved_schema = resolve_schema_references(resolved_schema, assembled_dir)
                
                # Write the fully resolved schema
                dest_file = assembled_entities_dir / schema_file.name
                with open(dest_file, 'w', encoding='utf-8') as f:
                    json.dump(fully_resolved_schema, f, indent=2, ensure_ascii=False)
                print(f"✅ Assembled entity {schema_file.name} (fully dereferenced)")
            else:
                # If loading failed, just copy the file as-is
                dest_file = assembled_entities_dir / schema_file.name
                shutil.copy2(schema_file, dest_file)
                print(f"✓ Copied entity {schema_file.name} (no processing)")

def assemble_schemas_from_manifest(manifest, templates_dir, snippets, snippets_dir, assembled_dir):
    """Assemble schemas based on the manifest configuration."""
    # Set up Jinja2 environment
    env = Environment(loader=FileSystemLoader(str(templates_dir)))
    
    # Process components
    if 'components' in manifest:
        components_dir = assembled_dir / 'components'
        components_dir.mkdir(parents=True, exist_ok=True)
        
        for component_name, config in manifest['components'].items():
            try:
                # Get template
                template_path = config.get('template', 'component.jinja')
                template = env.get_template(template_path)
                
                # Resolve snippet references in the configuration
                resolved_config = resolve_snippet_references(config, snippets, snippets_dir)
                resolved_config['component_name'] = component_name
                
                # Render template
                rendered = template.render(**resolved_config)
                
                # Parse as JSON to validate
                schema_data = json.loads(rendered)
                
                # Write to file
                output_file = components_dir / f"{component_name}.schema.json"
                with open(output_file, 'w', encoding='utf-8') as f:
                    json.dump(schema_data, f, indent=2, ensure_ascii=False)
                
                print(f"✅ Assembled {output_file}")
                
            except Exception as e:
                print(f"❌ Error assembling {component_name}: {e}")

def main():
    """Main assembly function."""
    # Define paths
    script_dir = Path(__file__).parent
    v3_dir = script_dir.parent
    schemas_dir = v3_dir / 'schemas'
    templates_dir = schemas_dir / 'templates'
    snippets_dir = schemas_dir / 'snippets'
    assembled_dir = schemas_dir / 'assembled'
    manifest_file = v3_dir / 'assembly_manifest.json'
    
    print("🧩 Assembling final schemas using Jinja2 templates...")
    
    # Load assembly manifest
    if not manifest_file.exists():
        print(f"❌ Assembly manifest not found: {manifest_file}")
        return 1
    
    manifest = load_json_file(manifest_file)
    if not manifest:
        print("❌ Failed to load assembly manifest")
        return 1
    
    # Load snippets
    snippets = load_snippets(snippets_dir)
    print(f"Loaded {len(snippets)} total snippets")
    
    # Clean and recreate assembled directory
    if assembled_dir.exists():
        shutil.rmtree(assembled_dir)
    assembled_dir.mkdir(parents=True, exist_ok=True)
    
    # Copy base schemas (now with snippet resolution)
    copy_base_schemas(schemas_dir, assembled_dir, snippets)
    
    # Assemble schemas from manifest (creates components)
    assemble_schemas_from_manifest(manifest, templates_dir, snippets, snippets_dir, assembled_dir)
    
    # Copy and process entity schemas (AFTER components are assembled)
    copy_entity_schemas(schemas_dir, assembled_dir, snippets)
    
    print("✅ Schema assembly complete.")
    return 0

if __name__ == "__main__":
    exit(main()) 