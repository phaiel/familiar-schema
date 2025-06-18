#!/usr/bin/env python3
"""
Snippet Opportunities Analysis for Familiar v3 Schemas

This script analyzes all schema files to identify opportunities for using
canonical snippets instead of inline definitions. It looks for patterns
that could be replaced with existing snippets or suggests new snippets.
"""

import os
import json
from pathlib import Path
from collections import defaultdict, Counter

def load_json_file(file_path):
    """Load and parse a JSON file."""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            return json.load(f)
    except Exception as e:
        print(f"Error loading {file_path}: {e}")
        return None

def get_all_snippets(snippets_dir):
    """Get all available snippets."""
    snippets = {}
    
    # Load field snippets
    fields_dir = snippets_dir / 'fields'
    if fields_dir.exists():
        for snippet_file in fields_dir.glob('*.json'):
            snippet_name = snippet_file.stem
            snippet_data = load_json_file(snippet_file)
            if snippet_data:
                snippets[f"fields/{snippet_name}"] = snippet_data
    
    # Load type snippets
    types_dir = snippets_dir / 'types'
    if types_dir.exists():
        for snippet_file in types_dir.rglob('*.json'):
            rel_path = snippet_file.relative_to(types_dir)
            snippet_name = str(rel_path).replace('.json', '')
            snippet_data = load_json_file(snippet_file)
            if snippet_data:
                snippets[f"types/{snippet_name}"] = snippet_data
    
    return snippets

def analyze_schema_for_opportunities(schema_data, schema_file, snippets):
    """Analyze a single schema for snippet opportunities."""
    opportunities = []
    
    def analyze_object(obj, path=""):
        if isinstance(obj, dict):
            # Check for UUID patterns
            if (obj.get("type") == "string" and 
                obj.get("format") == "uuid"):
                # Check if this could use EntityId, UserId, or TenantId
                field_name = path.split('.')[-1] if path else ""
                if "entity_id" in field_name.lower():
                    opportunities.append({
                        "type": "field_replacement",
                        "priority": "high",
                        "field_path": path,
                        "current": obj,
                        "suggested_snippet": "fields/EntityId",
                        "reason": "UUID field for entity ID should use EntityId.json snippet"
                    })
                elif "user_id" in field_name.lower():
                    opportunities.append({
                        "type": "field_replacement", 
                        "priority": "high",
                        "field_path": path,
                        "current": obj,
                        "suggested_snippet": "fields/UserId",
                        "reason": "UUID field for user ID should use UserId.json snippet"
                    })
                elif "tenant_id" in field_name.lower():
                    opportunities.append({
                        "type": "field_replacement",
                        "priority": "high", 
                        "field_path": path,
                        "current": obj,
                        "suggested_snippet": "fields/TenantId",
                        "reason": "UUID field for tenant ID should use TenantId.json snippet"
                    })
                else:
                    opportunities.append({
                        "type": "field_replacement",
                        "priority": "medium",
                        "field_path": path,
                        "current": obj,
                        "suggested_snippet": "types/primitives/UUID",
                        "reason": "Generic UUID field should use UUID.json snippet"
                    })
            
            # Check for date-time patterns
            elif (obj.get("type") == "string" and 
                  obj.get("format") == "date-time"):
                field_name = path.split('.')[-1] if path else ""
                if "created_at" in field_name.lower():
                    opportunities.append({
                        "type": "field_replacement",
                        "priority": "high",
                        "field_path": path,
                        "current": obj,
                        "suggested_snippet": "fields/CreatedAt",
                        "reason": "Creation timestamp should use CreatedAt.json snippet"
                    })
                elif "completed_at" in field_name.lower() or "finished_at" in field_name.lower():
                    opportunities.append({
                        "type": "field_replacement",
                        "priority": "high",
                        "field_path": path,
                        "current": obj,
                        "suggested_snippet": "fields/CompletedAt",
                        "reason": "Completion timestamp should use CompletedAt.json snippet"
                    })
                elif "start" in field_name.lower() and "date" in field_name.lower():
                    opportunities.append({
                        "type": "field_replacement",
                        "priority": "high",
                        "field_path": path,
                        "current": obj,
                        "suggested_snippet": "fields/StartDate",
                        "reason": "Start date should use StartDate.json snippet"
                    })
                elif "end" in field_name.lower() and "date" in field_name.lower():
                    opportunities.append({
                        "type": "field_replacement",
                        "priority": "high",
                        "field_path": path,
                        "current": obj,
                        "suggested_snippet": "fields/EndDate",
                        "reason": "End date should use EndDate.json snippet"
                    })
                else:
                    opportunities.append({
                        "type": "field_replacement",
                        "priority": "medium",
                        "field_path": path,
                        "current": obj,
                        "suggested_snippet": "types/primitives/Timestamp",
                        "reason": "Generic timestamp should use Timestamp.json snippet"
                    })
            
            # Check for enum patterns that could be standardized
            elif "enum" in obj:
                enum_values = obj["enum"]
                field_name = path.split('.')[-1] if path else ""
                
                # Check for exact matches with existing enum snippets
                for snippet_key, snippet_data in snippets.items():
                    if (snippet_data.get("type") == "string" and 
                        "enum" in snippet_data and 
                        snippet_data["enum"] == enum_values):
                        opportunities.append({
                            "type": "enum_standardization",
                            "priority": "high",
                            "field_path": path,
                            "current": obj,
                            "suggested_snippet": snippet_key,
                            "reason": f"Enum exactly matches existing {snippet_key} snippet"
                        })
                        break  # Found exact match, don't check other patterns
                else:
                    # No exact match found, check for partial patterns
                    # Thread state patterns
                    if any(val in ["active", "inactive", "archived", "suspended"] for val in enum_values):
                        if "types/lifecycles/ThreadState" in snippets:
                            opportunities.append({
                                "type": "enum_standardization",
                                "priority": "medium",
                                "field_path": path,
                                "current": obj,
                                "suggested_snippet": "types/lifecycles/ThreadState",
                                "reason": "Thread state enum should use standard ThreadState.json snippet"
                            })
                    
                    # Bond state patterns  
                    elif any(val in ["forming", "active", "weakening", "dissolved"] for val in enum_values):
                        if "types/lifecycles/BondState" in snippets:
                            opportunities.append({
                                "type": "enum_standardization",
                                "priority": "medium",
                                "field_path": path,
                                "current": obj,
                                "suggested_snippet": "types/lifecycles/BondState",
                                "reason": "Bond state enum should use standard BondState.json snippet"
                            })
            
            # Check for common field patterns
            elif obj.get("type") == "string":
                field_name = path.split('.')[-1] if path else ""
                if field_name.lower() == "name" and not obj.get("pattern"):
                    opportunities.append({
                        "type": "field_replacement",
                        "priority": "low",
                        "field_path": path,
                        "current": obj,
                        "suggested_snippet": "fields/Name",
                        "reason": "Name field should use Name.json snippet for consistency"
                    })
                elif field_name.lower() == "description" and not obj.get("minLength"):
                    opportunities.append({
                        "type": "field_replacement",
                        "priority": "low",
                        "field_path": path,
                        "current": obj,
                        "suggested_snippet": "fields/Description",
                        "reason": "Description field should use Description.json snippet for consistency"
                    })
            
            # Check for generic objects that could be typed
            elif obj.get("type") == "object" and not obj.get("properties") and not obj.get("$ref"):
                opportunities.append({
                    "type": "generic_object",
                    "priority": "medium",
                    "field_path": path,
                    "current": obj,
                    "suggested_snippet": None,
                    "reason": "Generic object could benefit from specific typing or snippet reference"
                })
            
            # Recursively analyze nested objects
            for key, value in obj.items():
                new_path = f"{path}.{key}" if path else key
                if key == "properties" and isinstance(value, dict):
                    for prop_key, prop_value in value.items():
                        analyze_object(prop_value, f"{new_path}.{prop_key}")
                else:
                    analyze_object(value, new_path)
        
        elif isinstance(obj, list):
            for i, item in enumerate(obj):
                analyze_object(item, f"{path}[{i}]")
    
    analyze_object(schema_data)
    return opportunities

def generate_summary_report(all_opportunities, total_files):
    """Generate a summary report of all opportunities."""
    
    # Count by priority
    priority_counts = Counter()
    type_counts = Counter()
    
    for file_path, opportunities in all_opportunities.items():
        for opp in opportunities:
            priority_counts[opp["priority"]] += 1
            type_counts[opp["type"]] += 1
    
    total_opportunities = sum(priority_counts.values())
    files_with_opportunities = len(all_opportunities)
    
    report = {
        "summary": {
            "total_files_analyzed": total_files,
            "files_with_opportunities": files_with_opportunities,
            "total_opportunities": total_opportunities,
            "opportunity_rate": f"{(files_with_opportunities / total_files * 100):.1f}%"
        },
        "by_priority": dict(priority_counts),
        "by_type": dict(type_counts),
        "detailed_opportunities": all_opportunities
    }
    
    return report

def main():
    """Main analysis function."""
    # Define paths
    script_dir = Path(__file__).parent
    schemas_dir = script_dir / 'schemas'
    snippets_dir = schemas_dir / 'snippets'
    
    print("🔍 Analyzing schemas for snippet opportunities...")
    
    # Load all available snippets
    snippets = get_all_snippets(snippets_dir)
    print(f"📦 Found {len(snippets)} available snippets")
    
    # Analyze all schema files
    all_opportunities = {}
    total_files = 0
    
    # Analyze source schemas (not assembled ones)
    for schema_file in schemas_dir.rglob("*.schema.json"):
        # Skip assembled schemas
        if "assembled" in str(schema_file):
            continue
            
        total_files += 1
        schema_data = load_json_file(schema_file)
        
        if schema_data:
            opportunities = analyze_schema_for_opportunities(schema_data, schema_file, snippets)
            if opportunities:
                rel_path = schema_file.relative_to(schemas_dir)
                all_opportunities[str(rel_path)] = opportunities
    
    # Generate report
    report = generate_summary_report(all_opportunities, total_files)
    
    # Save detailed report
    report_file = script_dir / 'snippet_opportunities_report.json'
    with open(report_file, 'w', encoding='utf-8') as f:
        json.dump(report, f, indent=2, ensure_ascii=False)
    
    # Print summary
    print(f"\n📊 SNIPPET OPPORTUNITIES ANALYSIS")
    print(f"{'='*50}")
    print(f"📁 Total files analyzed: {report['summary']['total_files_analyzed']}")
    print(f"🎯 Files with opportunities: {report['summary']['files_with_opportunities']}")
    print(f"📈 Opportunity rate: {report['summary']['opportunity_rate']}")
    print(f"🔢 Total opportunities: {report['summary']['total_opportunities']}")
    
    print(f"\n🎯 BY PRIORITY:")
    for priority in ['high', 'medium', 'low']:
        count = report['by_priority'].get(priority, 0)
        print(f"  {priority.upper()}: {count}")
    
    print(f"\n📋 BY TYPE:")
    for opp_type, count in report['by_type'].items():
        print(f"  {opp_type.replace('_', ' ').title()}: {count}")
    
    # Show top opportunities
    print(f"\n🔥 TOP OPPORTUNITIES:")
    high_priority_files = []
    for file_path, opportunities in all_opportunities.items():
        high_priority_count = sum(1 for opp in opportunities if opp["priority"] == "high")
        if high_priority_count > 0:
            high_priority_files.append((file_path, high_priority_count))
    
    high_priority_files.sort(key=lambda x: x[1], reverse=True)
    for file_path, count in high_priority_files[:10]:
        print(f"  📄 {file_path}: {count} high-priority opportunities")
    
    print(f"\n💾 Detailed report saved to: {report_file}")
    
    return 0

if __name__ == "__main__":
    exit(main()) 