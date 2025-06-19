#!/usr/bin/env python3
"""
Schema Title Injector for Familiar v3

This script programmatically adds 'title' properties to anonymous, inline
object definitions within a directory of JSON schemas. This is a crucial
step to ensure that code generation tools like schemafy or typify produce
clean, human-readable type names instead of long, path-based names.

It generates titles by combining the parent schema's title with the
property name of the anonymous object, converting them to PascalCase.

Example:
An anonymous object under the 'fields' property in a schema with
'title': 'BondContent' will be given the title 'BondContentFields'.
"""
import json
from pathlib import Path
import re
import argparse

def to_pascal_case(s: str) -> str:
    """Converts a snake_case or kebab-case string to PascalCase."""
    return ''.join(word.capitalize() for word in re.split('_|-', s))

def clean_title_for_concatenation(title: str) -> str:
    """Cleans a title to be suitable for concatenation by removing common suffixes and spaces."""
    # Remove common suffixes that would be redundant
    cleaned = title.replace(" Component", "").replace(" Entity", "").replace(" Payload", "")
    cleaned = cleaned.replace(" Event", "").replace(" Table", "").replace(" Field", "")
    cleaned = cleaned.replace(" Type", "").replace(" Schema", "").replace(" Definition", "")
    
    # Remove all spaces and convert to PascalCase
    cleaned = ''.join(word.capitalize() for word in cleaned.split())
    
    return cleaned

def add_titles_recursive(node: any, parent_title: str):
    """
    Recursively traverses the schema, adding 'title' to anonymous objects.
    """
    if isinstance(node, dict):
        # Determine the current title context
        current_title = node.get("title", parent_title)
        if not isinstance(current_title, str):
             current_title = parent_title

        for key, value in node.items():
            if isinstance(value, dict):
                # This is an anonymous object if it's a schema-like object
                is_schema_object = "properties" in value or "type" in value
                if is_schema_object and value.get("type") == "object":
                    # Generate a new title and inject it
                    clean_parent = clean_title_for_concatenation(current_title)
                    pascal_key = to_pascal_case(key)
                    new_title = f"{clean_parent}{pascal_key}"
                    
                    # Check if title needs to be added or updated
                    existing_title = value.get("title")
                    if not existing_title:
                        value["title"] = new_title
                        print(f"  - Injected title '{new_title}' for key '{key}'")
                    elif existing_title != new_title and (" " in existing_title or existing_title.endswith("ComponentFields") or existing_title.endswith("ComponentPhysicsProperties")):
                        # Update titles that have spaces or are malformed
                        value["title"] = new_title
                        print(f"  - Updated title from '{existing_title}' to '{new_title}' for key '{key}'")
                
                # Recurse into the child node
                child_title = value.get("title", new_title if 'new_title' in locals() else current_title)
                add_titles_recursive(value, child_title)

            elif isinstance(value, list):
                # Recurse into list items
                for i, item in enumerate(value):
                    # For items in an array, we can generate a title based on the key and index
                    clean_parent = clean_title_for_concatenation(current_title)
                    pascal_key = to_pascal_case(key)
                    item_parent_title = f"{clean_parent}{pascal_key}Item"
                    add_titles_recursive(item, item_parent_title)

    elif isinstance(node, list):
        for item in node:
            add_titles_recursive(item, parent_title)


def process_schema_file(file_path: Path):
    """Reads, processes, and writes a single schema file."""
    try:
        with file_path.open('r', encoding='utf-8') as f:
            schema = json.load(f)
            
        # Use the file's stem as the initial root title if no title exists
        root_title = schema.get("title", to_pascal_case(file_path.stem.replace('.schema', '')))
        
        print(f"\nProcessing {file_path.name} (Root Title: '{root_title}')...")
        
        # Store original schema for comparison
        original_schema = json.dumps(schema, sort_keys=True)
        
        add_titles_recursive(schema, root_title)
        
        # Only write if changes were made
        new_schema = json.dumps(schema, sort_keys=True)
        if original_schema != new_schema:
            with file_path.open('w', encoding='utf-8') as f:
                json.dump(schema, f, indent=2, ensure_ascii=False)
            print(f"  ✅ Updated {file_path.name}")
        else:
            print(f"  ⏭️  No changes needed for {file_path.name}")
            
    except json.JSONDecodeError:
        print(f"  - ⚠️  Skipping {file_path.name}, not a valid JSON file.")
    except Exception as e:
        print(f"  - ❌ Error processing {file_path.name}: {e}")


def main(schemas_dir: Path):
    """Main function to process all schemas in a directory."""
    print("=== Starting Schema Title Injection ===")
    print(f"Target directory: {schemas_dir.resolve()}")
    
    if not schemas_dir.is_dir():
        print("❌ Error: Provided path is not a directory.")
        return

    schema_files = list(schemas_dir.rglob("*.json"))
    processed_count = 0
    
    for schema_file in schema_files:
        if ".vscode" in schema_file.parts or "assembled" in schema_file.parts:
            continue  # Skip VSCode settings and assembled files
        process_schema_file(schema_file)
        processed_count += 1
        
    print(f"\n=== Schema Title Injection Complete ===")
    print(f"Processed {processed_count} schema files.")


if __name__ == "__main__":
    # The project root is three levels up from this script
    project_root = Path(__file__).parent.parent
    
    parser = argparse.ArgumentParser(description="Inject titles into anonymous JSON schema objects.")
    parser.add_argument(
        "--schemas-dir",
        type=Path,
        default=project_root / "schemas",
        help="Path to the root schemas directory to process."
    )
    
    args = parser.parse_args()
    main(args.schemas_dir) 