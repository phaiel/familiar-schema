#!/usr/bin/env python3
"""
Familiar v3 Schema Assembly & Dereferencing Pipeline (Robust `allOf` Handling)

This script uses the standard `referencing` library to robustly resolve all
`$ref` and `allOf` directives, creating fully self-contained schemas suitable
for code generation and other tooling.
"""
import json
import os
import shutil
import argparse
from pathlib import Path
from referencing import Registry, Resource
from referencing.jsonschema import DRAFT202012
from referencing.exceptions import Unresolvable

def deep_merge(d1, d2):
    """Recursively merges dictionary d2 into d1, handling lists."""
    for k, v in d2.items():
        if k in d1 and isinstance(d1[k], dict) and isinstance(v, dict):
            d1[k] = deep_merge(d1[k], v)
        elif k in d1 and isinstance(d1[k], list) and isinstance(v, list):
            d1[k].extend(x for x in v if x not in d1[k])
        else:
            d1[k] = v
    return d1

def _dereference_recursive(node, resolver):
    """
    Recursively walk a JSON structure, resolve all $ref pointers,
    and merge allOf directives.
    """
    if isinstance(node, dict):
        if "$ref" in node:
            ref_value = node["$ref"]
            if not isinstance(ref_value, str):
                return node

            try:
                resolved = resolver.lookup(ref_value)
                return _dereference_recursive(resolved.contents, resolved.resolver)
            except Unresolvable as e:
                print(f"  ↳ ❌ Unresolvable reference: '{ref_value}'. Error: {e}")
                return None
            except Exception as e:
                print(f"  ↳ ❌ Unexpected error resolving reference '{ref_value}': {e}")
                return None

        if "allOf" in node:
            merged = {}
            for sub_schema in node["allOf"]:
                dereferenced_sub = _dereference_recursive(sub_schema, resolver)
                if dereferenced_sub:
                    deep_merge(merged, dereferenced_sub)
            
            original_node = node.copy()
            original_node.pop("allOf")
            
            dereferenced_original = _dereference_recursive(original_node, resolver)
            if dereferenced_original:
                deep_merge(merged, dereferenced_original)
            return merged

        return {k: _dereference_recursive(v, resolver) for k, v in node.items()}
    
    elif isinstance(node, list):
        return [_dereference_recursive(item, resolver) for item in node]

    return node

def dereference_schema(schema_path: Path, registry: Registry) -> dict:
    """
    Loads a schema and fully dereferences it, including all $ref and allOf.
    """
    try:
        schema_content = json.loads(schema_path.read_text(encoding="utf-8"))
        resolver = registry.resolver(schema_path.as_uri())
        return _dereference_recursive(schema_content, resolver)
    except Exception as e:
        print(f"❌ Error processing {schema_path.name}: {e}")
        return None

def create_assembled_root_schema(source_dir: Path, output_dir: Path):
    """
    Creates a new root FamiliarTypes.schema.json in the assembled directory.
    This new root file will contain the actual content of each assembled schema
    as properties of the root object, making it compatible with schemafy.
    """
    source_root_path = source_dir / "FamiliarTypes.schema.json"
    if not source_root_path.exists():
        print(f"  ↳ ⚠️  Source root schema not found at {source_root_path}. Skipping root schema generation.")
        return

    print(f"  -> Generating new root schema in {output_dir}")
    source_root_schema = json.loads(source_root_path.read_text(encoding="utf-8"))
    
    new_root_schema = {
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": source_root_schema.get('$id', 'https://familiar.dev/schemas/FamiliarTypes.v1.schema.json'),
        "title": "FamiliarTypes",
        "description": "A root schema that defines all top-level types for the Familiar system.",
        "type": "object",
        "properties": {}
    }

    # For each type in the original root schema, load the assembled version and add it as a property
    for type_name, ref_obj in source_root_schema.get("$defs", {}).items():
        ref_path = ref_obj.get("$ref")
        if ref_path:
            # Convert the relative path to the assembled file path
            assembled_file_path = output_dir / ref_path.lstrip("./")
            
            if assembled_file_path.exists():
                try:
                    assembled_content = json.loads(assembled_file_path.read_text(encoding="utf-8"))
                    # Add the assembled content as a property of the root object
                    new_root_schema["properties"][type_name] = assembled_content
                    print(f"    ↳ Added {type_name} as property from {assembled_file_path.relative_to(output_dir)}")
                except Exception as e:
                    print(f"    ↳ ⚠️  Failed to add {type_name}: {e}")
            else:
                print(f"    ↳ ⚠️  Assembled file not found: {assembled_file_path}")

    output_root_path = output_dir / "FamiliarTypes.schema.json"
    with open(output_root_path, 'w', encoding='utf-8') as f:
        json.dump(new_root_schema, f, indent=2, ensure_ascii=False)
    print(f"  ↳ ✅ New root schema created at {output_root_path.relative_to(output_dir.parent.parent)}")


def main(args):
    """Main execution function."""
    source_dir = args.source_dir.resolve()
    output_dir = args.output_dir.resolve()
    
    print(f"Source directory: {source_dir}")
    print(f"Output directory: {output_dir}")

    if output_dir.exists():
        shutil.rmtree(output_dir)
    output_dir.mkdir(parents=True, exist_ok=True)

    print("\n➡️  Pass 1: Loading all schemas into a registry...")
    all_schemas = list(source_dir.rglob('*.json'))
    resources = []
    for schema_path in all_schemas:
        try:
            contents = json.loads(schema_path.read_text(encoding="utf-8"))
            resource = Resource.from_contents(contents, default_specification=DRAFT202012)
            resources.append((schema_path.as_uri(), resource))
        except Exception as e:
            print(f"  ↳ ⚠️  Could not load {schema_path.name}: {e}")

    registry = Registry().with_resources(resources)
    print(f"✅ Loaded {len(resources)} schemas into the registry.")

    print("\n➡️  Pass 2: Dereferencing all schema references...")
    all_source_schemas = [p for p in source_dir.rglob('*.schema.json') if "FamiliarTypes" not in p.name]
    
    processed_count = 0
    for schema_path in all_source_schemas:
        print(f"  -> Processing {schema_path.relative_to(source_dir)}")
        final_schema = dereference_schema(schema_path, registry)
        
        if final_schema:
            final_schema.pop('$schema', None)
            final_schema.pop('$id', None)
            
            relative_path = schema_path.relative_to(source_dir)
            output_path = output_dir / relative_path
            output_path.parent.mkdir(parents=True, exist_ok=True)
            
            with open(output_path, 'w', encoding='utf-8') as f:
                json.dump(final_schema, f, indent=2, ensure_ascii=False)
            processed_count += 1
        else:
            print(f"  ↳ ⚠️  Skipping {schema_path.name} due to processing errors.")

    print("\n➡️  Pass 3: Generating new root schema...")
    create_assembled_root_schema(source_dir, output_dir)

    print(f"\n✅ Schema assembly and dereferencing complete. {processed_count} schemas processed.")

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Familiar v3 Schema Assembler")
    parser.add_argument(
        "--source-dir",
        type=Path,
        default=Path(__file__).parent.parent / "schemas",
        help="The directory containing the source JSON schemas."
    )
    parser.add_argument(
        "--output-dir",
        type=Path,
        default=Path(__file__).parent.parent / "schemas" / "assembled",
        help="The directory where assembled schemas will be written."
    )
    args = parser.parse_args()
    main(args) 