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

def resolve_references(data, base_dir, cache):
    """
    Recursively resolve $ref references by inlining content.
    - data: The data to process (dict or list).
    - base_dir: The directory context for resolving relative file paths.
    - cache: A dictionary to cache loaded JSON files to avoid re-reading.
    """
    if isinstance(data, dict):
        if '$ref' in data and isinstance(data['$ref'], str):
            ref_path_str = data['$ref']
            
            # Use pathlib to resolve the absolute path
            # base_dir is the directory of the file containing the $ref
            ref_path = (base_dir / ref_path_str).resolve()

            if ref_path in cache:
                return cache[ref_path]

            if not ref_path.is_file():
                print(f"⚠️ Warning: Referenced file not found: {ref_path}")
                return data # Return original ref if not found

            try:
                with open(ref_path, 'r', encoding='utf-8') as f:
                    ref_content = json.load(f)
                
                # The new base for nested refs is the directory of the referenced file
                new_base_dir = ref_path.parent
                
                # Recursively resolve references within the loaded content
                resolved_content = resolve_references(ref_content, new_base_dir, cache)
                
                # Cache the fully resolved content
                cache[ref_path] = resolved_content
                return resolved_content
            except json.JSONDecodeError:
                print(f"⚠️ Warning: Could not decode JSON from {ref_path}")
                return data # Return original ref on error
            except Exception as e:
                print(f"⚠️ Warning: Error processing $ref {ref_path_str} from {base_dir}: {e}")
                return data
        
        # If no $ref, or $ref is not a string, recurse into dictionary values
        return {key: resolve_references(value, base_dir, cache) for key, value in data.items()}
    
    elif isinstance(data, list):
        return [resolve_references(item, base_dir, cache) for item in data]
        
    else:
        return data


def fully_dereference_schema(schema_path: Path, cache: dict):
    """
    Loads a schema and resolves all local and file-based $ref references,
    including those within allOf, until no more references can be resolved.
    """
    if not schema_path.is_file():
        return None
    
    # The initial base directory is the parent of the schema file itself
    base_dir = schema_path.parent
    
    # Load the initial schema content
    try:
        with open(schema_path, 'r', encoding='utf-8') as f:
            current_schema = json.load(f)
    except Exception as e:
        print(f"Error loading initial schema {schema_path}: {e}")
        return None

    # Multi-pass dereferencing to handle nested references
    max_passes = 10
    for i in range(max_passes):
        # print(f"  -> Dereferencing Pass {i+1} for {schema_path.name}...")
        previous_state = json.dumps(current_schema, sort_keys=True)
        
        # Resolve all $ref pointers (both file and internal)
        # We pass the file's parent dir as the base for resolving file refs
        current_schema = resolve_references(current_schema, base_dir, cache)
        
        # Custom logic for allOf after general $ref resolution
        if 'allOf' in current_schema:
            new_schema = {}
            new_properties = {}
            required_properties = []

            for sub_schema in current_schema['allOf']:
                # The sub_schema should already be resolved by the call above
                # Merge properties
                if 'properties' in sub_schema:
                    new_properties.update(sub_schema['properties'])
                # Merge required fields
                if 'required' in sub_schema:
                    required_properties.extend(sub_schema['required'])
                # Merge other top-level keys
                for key, value in sub_schema.items():
                    if key not in ['properties', 'required', 'allOf', '$id', '$schema', 'title', 'description']:
                        if key not in new_schema:
                            new_schema[key] = value
                        # Note: This is a simple merge. More complex logic may be needed
                        # for conflicting keys.
            
            # Combine with original schema's top-level keys
            for key, value in current_schema.items():
                 if key not in ['allOf', 'properties', 'required']:
                    new_schema[key] = value
            
            if new_properties:
                # If the original schema also had properties, merge them
                # giving precedence to the original's properties
                original_properties = current_schema.get('properties', {})
                original_properties.update(new_properties)
                new_schema['properties'] = original_properties
            
            if required_properties:
                # Also merge required properties from original schema
                original_required = current_schema.get('required', [])
                original_required.extend(required_properties)
                new_schema['required'] = sorted(list(set(original_required)))

            current_schema = new_schema


        current_state = json.dumps(current_schema, sort_keys=True)

        if previous_state == current_state:
            # print("  -> No more `allOf` references to resolve.")
            break
    else:
        print(f"⚠️ Warning: Max dereferencing passes reached for {schema_path.name}")
        
    return current_schema


def copy_base_schemas(schemas_dir, assembled_dir, snippets):
    """Copy and process base schemas to assembled directory."""
    base_dir = schemas_dir / '_base'
    # Base schemas don't need assembly, just copying.
    # The new dereferencing logic handles them when they are referenced.
    if base_dir.exists():
        for schema_file in base_dir.glob('*.schema.json'):
            shutil.copy2(schema_file, assembled_dir / schema_file.name)
            print(f"✓ Copied base schema {schema_file.name}")


def copy_entity_base_schemas(schemas_dir, assembled_dir, snippets):
    """Copy and process base schemas to assembled directory."""
    base_dir = schemas_dir / 'entities' / '_base'
    # Base schemas don't need assembly, just copying.
    # The new dereferencing logic handles them when they are referenced.
    if base_dir.exists():
        for schema_file in base_dir.glob('*.schema.json'):
            dest_dir = assembled_dir / 'entities' / '_base'
            dest_dir.mkdir(parents=True, exist_ok=True)
            shutil.copy2(schema_file, dest_dir / schema_file.name)
            print(f"✓ Copied entity base schema {schema_file.name}")


def assemble_and_dereference(schema_path: Path, assembled_dir: Path, jinja_env: Environment, snippets: dict, cache: dict):
    """Assemble a schema from a Jinja template, then fully dereference it."""
    
    # Step 1: Assemble the schema from its Jinja2 template
    template_name = str(schema_path.relative_to(schema_path.parent.parent))
    template = jinja_env.get_template(template_name)
    # Note: snippets are passed to Jinja context, though the new model prefers $ref
    assembled_content_str = template.render(snippets=snippets)
    
    try:
        assembled_schema = json.loads(assembled_content_str)
    except json.JSONDecodeError as e:
        print(f"❌ Error decoding assembled JSON for {schema_path.name}: {e}")
        return

    # Step 2: Write the intermediate assembled schema (for debugging)
    # The path inside 'assembled' should mirror the path inside 'schemas'
    relative_path = schema_path.relative_to(schemas_dir)
    assembled_path = assembled_dir / relative_path
    assembled_path.parent.mkdir(parents=True, exist_ok=True)
    with open(assembled_path, 'w', encoding='utf-8') as f:
        json.dump(assembled_schema, f, indent=2)

    # Step 3: Fully dereference the assembled schema
    # The base_dir for resolving $refs is the directory of the *assembled* file
    final_schema = fully_dereference_schema(assembled_path, cache)

    # Step 4: Write the final, dereferenced schema back to the same path
    if final_schema:
        with open(assembled_path, 'w', encoding='utf-8') as f:
            json.dump(final_schema, f, indent=2)
        print(f"✅ Assembled and dereferenced {relative_path}")


def main():
    """Main function to run the schema assembly process."""
    print("🧩 Assembling final schemas using Jinja2 templates...")
    # Get project root assuming script is in docs/v3/scripts
    project_root = Path(__file__).parent.parent.parent.parent
    schemas_dir = project_root / 'docs/v3/schemas'
    assembled_dir = schemas_dir / 'assembled'
    templates_dir = schemas_dir
    
    # Initialize Jinja2 environment
    jinja_env = Environment(
        loader=FileSystemLoader(str(templates_dir)),
        trim_blocks=True,
        lstrip_blocks=True
    )

    # Clean and recreate the assembled directory
    if assembled_dir.exists():
        shutil.rmtree(assembled_dir)
    assembled_dir.mkdir(parents=True)
    
    # --- New Dereferencing Flow ---
    
    # A cache to hold resolved file contents
    resolution_cache = {}

    # 1. Copy all base schemas first, so they are available for referencing
    copy_base_schemas(schemas_dir, assembled_dir, None)
    copy_entity_base_schemas(schemas_dir, assembled_dir, None)

    # 2. Iterate through all schema files that are NOT in a _base directory
    all_schemas = [p for p in schemas_dir.rglob('*.schema.json') if '_base' not in p.parts]

    for schema_path in all_schemas:
        # The base directory for resolving file references is the schema's original location
        base_dir = schema_path.parent
        
        # Load the raw schema file (it might be a Jinja template or plain JSON)
        try:
            with open(schema_path, 'r', encoding='utf-8') as f:
                content = f.read()
            # Try to render as a Jinja template. If it's not a template, it will pass through.
            template = jinja_env.from_string(content)
            rendered_content_str = template.render()
            initial_schema = json.loads(rendered_content_str)
        except Exception as e:
            print(f"➡️  Skipping non-template or invalid JSON file: {schema_path.name}")
            # If it's not a template or not a schema, just copy it
            relative_path = schema_path.relative_to(schemas_dir)
            dest_path = assembled_dir / relative_path
            dest_path.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(schema_path, dest_path)
            continue
            
        # Create a temporary file for the initial rendered content to act as a stable base
        temp_path = assembled_dir / f"_temp_{schema_path.name}"
        with open(temp_path, 'w', encoding='utf-8') as f:
            json.dump(initial_schema, f)

        # Fully dereference the schema, using the temp file's location as the base
        final_schema = fully_dereference_schema(temp_path, resolution_cache)
        
        # Remove the temp file
        os.remove(temp_path)

        # Write the final, fully dereferenced schema to the assembled directory
        if final_schema:
            relative_path = schema_path.relative_to(schemas_dir)
            output_path = assembled_dir / relative_path
            output_path.parent.mkdir(parents=True, exist_ok=True)
            with open(output_path, 'w', encoding='utf-8') as f:
                json.dump(final_schema, f, indent=2)
            print(f"✅ Assembled {relative_path} (fully dereferenced)")

    print("🔄 Resolving `allOf` inheritance for quicktype compatibility...")
    print("  -> (Handled during initial dereferencing)")
    print("✅ Schema assembly and dereferencing complete.")


if __name__ == "__main__":
    main()