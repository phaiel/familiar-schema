#!/usr/bin/env python3
"""
Contract Test Runner - SonarQube Compatible

Runs our rigorous contract tests and generates reports compatible with SonarQube.
This bypasses pytest compatibility issues while providing validation.
"""

import json
import time
from pathlib import Path
from tests.test_contracts import ContractValidator

def run_all_contract_tests():
    """Run all contract tests and generate reports."""
    
    print("🚀 Starting Rigorous Contract Testing Suite")
    print("=" * 60)
    
    start_time = time.time()
    validator = ContractValidator()
    
    # Basic validation
    print(f"📊 Loaded {len(validator.source_schemas)} concrete schemas (base schemas excluded)")
    assert len(validator.source_schemas) >= 115, f"Expected 115+ concrete schemas, found {len(validator.source_schemas)}"
    
    # Test critical enums
    print("\n🔍 Testing Critical Enums...")
    critical_enums = ["ThreadState", "BondState", "Priority", "EntityType", "AccessType"]
    enum_results = {}
    
    for enum_name in critical_enums:
        result = validator.validate_model_against_source(enum_name)
        enum_results[enum_name] = result
        status = "✅" if result["valid"] else "❌"
        print(f"   {status} {enum_name}: {result.get('type', 'unknown')}")
        
        if not result["valid"]:
            print(f"      Error: {result.get('error', 'Unknown')}")
        
        # Critical enums must pass
        assert result["valid"], f"Critical enum {enum_name} failed: {result.get('error')}"
        assert result.get("type") == "enum", f"{enum_name} should be an enum"
    
    # Test critical entities  
    print("\n🏗️  Testing Critical Entities...")
    critical_entities = ["Bond", "Thread", "Moment", "Filament", "Focus"]
    entity_results = {}
    
    for entity_name in critical_entities:
        result = validator.validate_model_against_source(entity_name)
        entity_results[entity_name] = result
        status = "✅" if result["valid"] else "❌"
        print(f"   {status} {entity_name}: {result.get('type', 'unknown')}")
        
        if not result["valid"]:
            print(f"      Error: {result.get('error', 'Unknown')}")
        
        # Critical entities must pass
        assert result["valid"], f"Critical entity {entity_name} failed: {result.get('error')}"
        assert result.get("type") == "pydantic", f"{entity_name} should be a Pydantic model"
    
    # Schema validation
    print("\n🔍 Running Validation...")
    validation_results = validator.run_validation()

    print(f"\n📊 CONTRACT TEST RESULTS:")
    print(f"   Total schemas: {validation_results['total_schemas']}")
    print(f"   Validated: {validation_results['validated']}")
    print(f"   Passed: {validation_results['passed']}")
    print(f"   Failed: {validation_results['failed']}")
    print(f"   Success rate: {validation_results['success_rate']:.1f}%")
    print(f"   By type: {validation_results['by_type']}")
    
    if validation_results['failures']:
        print(f"\n❌ FAILURES ({len(validation_results['failures'])}):")
        for i, failure in enumerate(validation_results['failures'][:10], 1):
            print(f"   {i:2d}. {failure['schema']:25} → {failure['error']}")
        if len(validation_results['failures']) > 10:
            print(f"   ... and {len(validation_results['failures']) - 10} more")
    
    # Generate SonarQube-compatible reports
    generate_sonarqube_reports(validation_results, enum_results, entity_results)
    
    # Validation requirements
    assert validation_results['success_rate'] >= 85.0, f"Success rate too low: {validation_results['success_rate']:.1f}%"
    assert validation_results['by_type']['pydantic'] > 0, "No Pydantic models found"
    assert validation_results['by_type']['enum'] > 0, "No Enums found"
    
    end_time = time.time()
    duration = end_time - start_time
    
    print(f"\n🎉 CONTRACT TESTING COMPLETE!")
    print(f"   Duration: {duration:.2f} seconds")
    print(f"   Success Rate: {validation_results['success_rate']:.1f}%")
    print(f"   Quality: {'EXCELLENT' if validation_results['success_rate'] >= 95 else 'GOOD' if validation_results['success_rate'] >= 85 else 'NEEDS IMPROVEMENT'}")
    
    return validation_results

def generate_sonarqube_reports(validation_results, enum_results, entity_results):
    """Generate SonarQube-compatible test reports."""
    
    reports_dir = Path("test-reports")
    reports_dir.mkdir(exist_ok=True)
    
    # JUnit XML format for SonarQube
    junit_xml = generate_junit_xml(validation_results, enum_results, entity_results)
    with open(reports_dir / "contract-tests.xml", "w") as f:
        f.write(junit_xml)
    
    # JSON summary for detailed analysis
    summary_report = {
        "timestamp": time.time(),
        "validation_results": validation_results,
        "critical_enums": enum_results,
        "critical_entities": entity_results,
        "quality_metrics": {
            "total_schemas": validation_results['total_schemas'],
            "success_rate": validation_results['success_rate'],
            "enum_coverage": validation_results['by_type']['enum'],
            "pydantic_coverage": validation_results['by_type']['pydantic'],
            "failure_rate": (validation_results['failed'] / validation_results['validated']) * 100
        }
    }
    
    with open(reports_dir / "contract-summary.json", "w") as f:
        json.dump(summary_report, f, indent=2)
    
    print(f"📄 Generated SonarQube reports in {reports_dir}/")

def generate_junit_xml(validation_results, enum_results, entity_results):
    """Generate JUnit XML format for SonarQube integration."""
    
    total_tests = len(enum_results) + len(entity_results) + 1  # +1 for validation test
    failures = len([r for r in enum_results.values() if not r['valid']]) + \
               len([r for r in entity_results.values() if not r['valid']])
    
    if validation_results['success_rate'] < 85.0:
        failures += 1
    
    xml = f'''<?xml version="1.0" encoding="UTF-8"?>
<testsuite name="ContractTests" tests="{total_tests}" failures="{failures}" time="0">
'''
    
    # Enum tests
    for name, result in enum_results.items():
        if result['valid']:
            xml += f'  <testcase name="test_critical_enum_{name}" classname="TestContracts" time="0"/>\n'
        else:
            xml += f'  <testcase name="test_critical_enum_{name}" classname="TestContracts" time="0">\n'
            xml += f'    <failure message="{result.get("error", "Unknown error")}">{result.get("error", "Unknown error")}</failure>\n'
            xml += f'  </testcase>\n'
    
    # Entity tests
    for name, result in entity_results.items():
        if result['valid']:
            xml += f'  <testcase name="test_critical_entity_{name}" classname="TestContracts" time="0"/>\n'
        else:
            xml += f'  <testcase name="test_critical_entity_{name}" classname="TestContracts" time="0">\n'
            xml += f'    <failure message="{result.get("error", "Unknown error")}">{result.get("error", "Unknown error")}</failure>\n'
            xml += f'  </testcase>\n'
    
    # Validation test
    if validation_results['success_rate'] >= 85.0:
        xml += f'  <testcase name="test_validation" classname="TestContracts" time="0"/>\n'
    else:
        xml += f'  <testcase name="test_validation" classname="TestContracts" time="0">\n'
        xml += f'    <failure message="Success rate too low: {validation_results["success_rate"]:.1f}%">Success rate {validation_results["success_rate"]:.1f}% below required 85%</failure>\n'
        xml += f'  </testcase>\n'
    
    xml += '</testsuite>\n'
    return xml

if __name__ == "__main__":
    try:
        results = run_all_contract_tests()
        exit(0)
    except AssertionError as e:
        print(f"\n💥 CONTRACT TEST FAILURE: {e}")
        exit(1)
    except Exception as e:
        print(f"\n💥 UNEXPECTED ERROR: {e}")
        import traceback
        traceback.print_exc()
        exit(1) 