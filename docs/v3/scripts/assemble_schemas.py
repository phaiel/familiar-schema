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
            elif ref_path.startswith('../') and any(snip_dir in ref_path for snip_dir in ['fields/', 'types/', 'workflow/', 'database/', 'primitives/', 'physics/', 'validation/', 'security/', 'social/']):
                # Handle references within snippets like "../fields/CompletedAt.json" or "../types/primitives/NullableTimestamp.json"
                snippet_key = ref_path.replace('../', '')
                # Ensure proper path structure for types
                if not snippet_key.startswith(('fields/', 'types/')):
                    # If it starts with a subdirectory name, it's likely a types reference
                    if any(snippet_key.startswith(subdir) for subdir in ['workflow/', 'database/', 'primitives/', 'physics/', 'validation/', 'security/', 'social/']):
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

def resolve_ref_references_multipass(obj, snippets, schemas_dir, max_passes=5):
    """Resolve $ref references with multiple passes to handle nested snippet chains."""
    current_obj = obj
    
    for pass_num in range(max_passes):
        # Convert to JSON and back to ensure we can detect changes
        before_json = json.dumps(current_obj, sort_keys=True)
        
        # Apply one pass of reference resolution
        current_obj = resolve_ref_references(current_obj, snippets, schemas_dir)
        
        # Check if anything changed
        after_json = json.dumps(current_obj, sort_keys=True)
        
        if before_json == after_json:
            # No changes made, we're done
            break
    
    return current_obj

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
                # Resolve $ref references to snippets with multi-pass
                resolved_schema = resolve_ref_references_multipass(schema_data, snippets, schemas_dir)
                
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

def resolve_schema_references(obj, assembled_dir, current_dir=None):
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
                    # Handle ./ references - these are relative to current directory
                    if ref_path.startswith('./_base/'):
                        # Entity references to ./_base/ should go to assembled _base/
                        clean_path = ref_path.replace('./_base/', '_base/')
                        schema_file = assembled_dir / clean_path
                    else:
                        # Other ./ references - use current directory context if available
                        clean_path = ref_path[2:]  # Remove './'
                        if current_dir:
                            # Resolve relative to current directory
                            schema_file = current_dir / clean_path
                        else:
                            # Fallback to _base/ assumption
                            schema_file = assembled_dir / '_base' / clean_path
                    
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
                        resolved_schema = resolve_schema_references(referenced_schema, assembled_dir, schema_file.parent)
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
            return {k: resolve_schema_references(v, assembled_dir, current_dir) for k, v in obj.items()}
    elif isinstance(obj, list):
        return [resolve_schema_references(item, assembled_dir, current_dir) for item in obj]
    else:
        return obj

def _resolve_allof(schema: dict, assembled_dir: Path, schema_dir: Path = None) -> dict:
    """
    Recursively resolves and inlines `$ref`s within an `allOf` array and other schema references.
    This ensures schemas are fully dereferenced for quicktype.
    """
    # First handle allOf references
    if 'allOf' in schema:
        new_properties = {}
        new_required = set()
        
        # Keep track of original properties to merge them last
        original_properties = schema.get('properties', {})
        original_required = set(schema.get('required', []))

        for item in schema['allOf']:
            if '$ref' in item:
                ref_path_str = item['$ref']
                # Construct the full path to the referenced schema in the assembled dir
                # Handle both ./_base/ and ../_base/ patterns
                if ref_path_str.startswith('../_base/'):
                    # Convert ../_base/ to _base/
                    ref_path_str = ref_path_str[3:]  # Remove '../'
                elif ref_path_str.startswith('./_base/'):
                    # Convert ./_base/ to _base/
                    ref_path_str = ref_path_str[2:]  # Remove './'
                elif ref_path_str.startswith('../'):
                    ref_path_str = ref_path_str[3:]  # Remove '../'
                elif ref_path_str.startswith('./'):
                    ref_path_str = ref_path_str[2:]  # Remove './'
                
                # Resolve path relative to schema directory if provided, otherwise assembled_dir
                if schema_dir and ref_path_str and not ref_path_str.startswith('_base/'):
                    ref_path = (schema_dir / ref_path_str).resolve()
                else:
                    ref_path = (assembled_dir / ref_path_str).resolve()

                if ref_path.exists():
                    ref_schema = load_json_file(ref_path)
                    if ref_schema:
                        # Recursively resolve the referenced schema first
                        resolved_ref = _resolve_allof(ref_schema, assembled_dir, ref_path.parent)
                        
                        # Merge properties and required fields
                        new_properties.update(resolved_ref.get('properties', {}))
                        new_required.update(resolved_ref.get('required', []))
                else:
                    print(f"⚠️  Warning: `allOf` reference not found: {ref_path}")
        
        # Merge the resolved properties with the original properties
        # Original properties take precedence in case of conflict
        new_properties.update(original_properties)
        schema['properties'] = new_properties
        
        # Merge required fields
        new_required.update(original_required)
        if new_required:
            schema['required'] = sorted(list(new_required))

        # Remove the allOf key as it has been processed
        del schema['allOf']
    
    # Also recursively resolve any other $ref patterns in the schema
    def resolve_refs_recursive(obj):
        if isinstance(obj, dict):
            if '$ref' in obj and len(obj) == 1:  # Only $ref, no other properties
                ref_path_str = obj['$ref']
                # Only resolve schema references, not snippet references
                if isinstance(ref_path_str, str) and ref_path_str.endswith('.schema.json'):
                    # Handle path resolution
                    if ref_path_str.startswith('./'):
                        # For ./ references, resolve relative to schema directory if available
                        ref_path_str = ref_path_str[2:]
                        if schema_dir:
                            ref_path = (schema_dir / ref_path_str).resolve()
                        else:
                            ref_path = (assembled_dir / '_base' / ref_path_str).resolve()
                    elif ref_path_str.startswith('../_base/'):
                        ref_path_str = ref_path_str[3:]
                        ref_path = (assembled_dir / ref_path_str).resolve()
                    elif ref_path_str.startswith('../'):
                        ref_path_str = ref_path_str[3:]
                        ref_path = (assembled_dir / ref_path_str).resolve()
                    else:
                        ref_path = (assembled_dir / ref_path_str).resolve()
                    if ref_path.exists():
                        ref_schema = load_json_file(ref_path)
                        if ref_schema:
                            resolved_ref = _resolve_allof(ref_schema, assembled_dir, ref_path.parent)
                            # Remove schema metadata to avoid conflicts
                            if '$id' in resolved_ref:
                                resolved_ref.pop('$id')
                            if '$schema' in resolved_ref:
                                resolved_ref.pop('$schema')
                            return resolved_ref
                return obj
            else:
                return {k: resolve_refs_recursive(v) for k, v in obj.items()}
        elif isinstance(obj, list):
            return [resolve_refs_recursive(item) for item in obj]
        else:
            return obj
    
    schema = resolve_refs_recursive(schema)
    return schema

def copy_entity_base_schemas(schemas_dir, assembled_dir, snippets):
    """Copy entity base schemas to assembled directory."""
    entity_base_dir = schemas_dir / 'entities' / '_base'
    assembled_base_dir = assembled_dir / '_base'
    
    # Ensure the assembled base directory exists
    assembled_base_dir.mkdir(parents=True, exist_ok=True)
    
    # Copy all entity base schema files, resolving snippet references
    if entity_base_dir.exists():
        for schema_file in entity_base_dir.glob('*.schema.json'):
            # Load the schema
            schema_data = load_json_file(schema_file)
            if schema_data:
                # Resolve $ref references to snippets with multi-pass
                resolved_schema = resolve_ref_references_multipass(schema_data, snippets, schemas_dir)
                
                # Write the processed schema
                dest_file = assembled_base_dir / schema_file.name
                with open(dest_file, 'w', encoding='utf-8') as f:
                    json.dump(resolved_schema, f, indent=2, ensure_ascii=False)
                print(f"✓ Copied entity base schema {schema_file.name}")
            else:
                # If loading failed, just copy the file as-is
                dest_file = assembled_base_dir / schema_file.name
                shutil.copy2(schema_file, dest_file)
                print(f"✓ Copied entity base schema {schema_file.name} (no processing)")

def copy_static_component_schemas(schemas_dir, assembled_dir, snippets, manifest):
    """Copy static component schemas that aren't generated from templates."""
    components_dir = schemas_dir / 'components'
    assembled_components_dir = assembled_dir / 'components'
    
    # Ensure the assembled components directory exists
    assembled_components_dir.mkdir(parents=True, exist_ok=True)
    
    # Get list of components that are generated from manifest
    manifest_components = set()
    if 'components' in manifest:
        manifest_components = set(f"{name}.schema.json" for name in manifest['components'].keys())
    
    # Copy all static component schema files, resolving snippet references
    if components_dir.exists():
        for schema_file in components_dir.glob('*.schema.json'):
            # Skip if this component is generated from the manifest
            if schema_file.name in manifest_components:
                continue
                
            # Load the schema
            schema_data = load_json_file(schema_file)
            if schema_data:
                # Resolve $ref references to snippets with multi-pass
                resolved_schema = resolve_ref_references_multipass(schema_data, snippets, schemas_dir)
                
                # Write the processed schema
                dest_file = assembled_components_dir / schema_file.name
                with open(dest_file, 'w', encoding='utf-8') as f:
                    json.dump(resolved_schema, f, indent=2, ensure_ascii=False)
                print(f"✓ Copied static component {schema_file.name}")
            else:
                # If loading failed, just copy the file as-is
                dest_file = assembled_components_dir / schema_file.name
                shutil.copy2(schema_file, dest_file)
                print(f"✓ Copied static component {schema_file.name} (no processing)")

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
                # First resolve snippet references with multi-pass
                resolved_schema = resolve_ref_references_multipass(schema_data, snippets, schemas_dir)
                
                # Then resolve schema references by inlining assembled schemas
                fully_resolved_schema = resolve_schema_references(resolved_schema, assembled_dir, assembled_entities_dir)
                
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

def copy_schema_directory(schemas_dir, assembled_dir, snippets, dir_name, full_dereference=False):
    """Generic function to copy and process schemas from any directory."""
    source_dir = schemas_dir / dir_name
    assembled_target_dir = assembled_dir / dir_name
    
    # Ensure the assembled target directory exists
    assembled_target_dir.mkdir(parents=True, exist_ok=True)
    
    # Copy all schema files, resolving references
    if source_dir.exists():
        for schema_file in source_dir.glob('*.schema.json'):
            # Load the schema
            schema_data = load_json_file(schema_file)
            if schema_data:
                # First resolve snippet references with multi-pass
                resolved_schema = resolve_ref_references_multipass(schema_data, snippets, schemas_dir)
                
                # Optionally resolve schema references by inlining (for entities)
                if full_dereference:
                    resolved_schema = resolve_schema_references(resolved_schema, assembled_dir, assembled_target_dir)
                
                # Write the processed schema
                dest_file = assembled_target_dir / schema_file.name
                with open(dest_file, 'w', encoding='utf-8') as f:
                    json.dump(resolved_schema, f, indent=2, ensure_ascii=False)
                
                status = "fully dereferenced" if full_dereference else "processed"
                print(f"✅ Assembled {dir_name}/{schema_file.name} ({status})")
            else:
                # If loading failed, just copy the file as-is
                dest_file = assembled_target_dir / schema_file.name
                shutil.copy2(schema_file, dest_file)
                print(f"✓ Copied {dir_name}/{schema_file.name} (no processing)")

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
    
    # Copy entity base schemas
    copy_entity_base_schemas(schemas_dir, assembled_dir, snippets)
    
    # Assemble schemas from manifest (creates components from templates)
    assemble_schemas_from_manifest(manifest, templates_dir, snippets, snippets_dir, assembled_dir)
    
    # Copy static component schemas (not generated from templates)
    copy_static_component_schemas(schemas_dir, assembled_dir, snippets, manifest)
    
    # Copy and process entity schemas (AFTER components are assembled)
    copy_entity_schemas(schemas_dir, assembled_dir, snippets)
    
    # Copy and process all other schema directories
    schema_directories = ['payloads', 'workflows', 'tables', 'events', 'laws', 'api']
    for dir_name in schema_directories:
        copy_schema_directory(schemas_dir, assembled_dir, snippets, dir_name, full_dereference=False)
    
    # Second Pass: Resolve allOf inheritance for full dereferencing
    print("\n🔄 Resolving `allOf` inheritance for quicktype compatibility...")
    
    all_assembled_schemas = list(assembled_dir.rglob("*.schema.json"))
    
    # Loop multiple times to handle nested inheritance
    for i in range(5):  # Up to 5 passes for deep inheritance
        print(f"  -> Dereferencing Pass {i+1}...")
        resolved_count = 0
        for schema_file in all_assembled_schemas:
            schema_data = load_json_file(schema_file)
            if schema_data and 'allOf' in schema_data:
                # Resolve the `allOf` references by inlining
                resolved_schema = _resolve_allof(schema_data, assembled_dir, schema_file.parent)
                
                # Write the fully resolved schema back to the same file
                with open(schema_file, 'w', encoding='utf-8') as f:
                    json.dump(resolved_schema, f, indent=2, ensure_ascii=False)
                resolved_count += 1
        
        if resolved_count == 0:
            print("  -> No more `allOf` references to resolve.")
            break  # Exit loop if no changes were made
    
    print("✅ Schema assembly and dereferencing complete.")
    return 0

if __name__ == "__main__":
    exit(main()) 