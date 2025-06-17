#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

console.log('🔍 Analyzing JSON Schema Dependencies...');
console.log('=====================================');

// Function to find all schema files
function findSchemaFiles(dir) {
  const files = [];
  const items = fs.readdirSync(dir);
  
  for (const item of items) {
    const fullPath = path.join(dir, item);
    const stat = fs.statSync(fullPath);
    
    if (stat.isDirectory()) {
      files.push(...findSchemaFiles(fullPath));
    } else if (item.endsWith('.schema.json')) {
      files.push(fullPath);
    }
  }
  
  return files;
}

// Function to extract $ref dependencies from a schema
function extractRefs(schemaPath) {
  try {
    const content = fs.readFileSync(schemaPath, 'utf8');
    const schema = JSON.parse(content);
    const refs = [];
    
    function findRefs(obj) {
      if (typeof obj === 'object' && obj !== null) {
        if (obj.$ref && typeof obj.$ref === 'string') {
          refs.push(obj.$ref);
        }
        for (const key in obj) {
          findRefs(obj[key]);
        }
      } else if (Array.isArray(obj)) {
        obj.forEach(findRefs);
      }
    }
    
    findRefs(schema);
    return refs;
  } catch (error) {
    console.error(`Error reading ${schemaPath}:`, error.message);
    return [];
  }
}

// Main analysis
const schemasDir = './schemas';
const schemaFiles = findSchemaFiles(schemasDir);

console.log(`📊 Found ${schemaFiles.length} schema files:`);
schemaFiles.forEach(file => console.log(`  - ${file}`));
console.log();

// Analyze dependencies
const dependencies = {};
const allRefs = new Set();

schemaFiles.forEach(schemaPath => {
  const relativePath = path.relative(schemasDir, schemaPath);
  const refs = extractRefs(schemaPath);
  dependencies[relativePath] = refs;
  refs.forEach(ref => allRefs.add(ref));
});

console.log('🔗 Schema Dependencies:');
console.log('======================');

for (const [schema, refs] of Object.entries(dependencies)) {
  console.log(`\n📄 ${schema}:`);
  if (refs.length === 0) {
    console.log('  ✅ No dependencies (self-contained)');
  } else {
    refs.forEach(ref => {
      // Check if the reference is local or external
      if (ref.startsWith('./') || ref.startsWith('../')) {
        console.log(`  🔗 Local: ${ref}`);
      } else if (ref.startsWith('http')) {
        console.log(`  🌐 External: ${ref}`);
      } else {
        console.log(`  ❓ Unknown: ${ref}`);
      }
    });
  }
}

console.log('\n📈 Dependency Graph (Mermaid Format):');
console.log('====================================');

// Generate Mermaid graph
console.log('```mermaid');
console.log('graph TD');

// Add nodes
schemaFiles.forEach(schemaPath => {
  const relativePath = path.relative(schemasDir, schemaPath);
  const nodeName = relativePath.replace(/[^a-zA-Z0-9]/g, '_');
  const displayName = path.basename(relativePath, '.schema.json');
  console.log(`  ${nodeName}["${displayName}"]`);
});

// Add edges
for (const [schema, refs] of Object.entries(dependencies)) {
  const fromNode = schema.replace(/[^a-zA-Z0-9]/g, '_');
  
  refs.forEach(ref => {
    if (ref.startsWith('./') || ref.startsWith('../')) {
      // Resolve relative path
      const refPath = path.resolve(path.dirname(path.join(schemasDir, schema)), ref);
      const refRelative = path.relative(schemasDir, refPath);
      const toNode = refRelative.replace(/[^a-zA-Z0-9]/g, '_');
      console.log(`  ${fromNode} --> ${toNode}`);
    }
  });
}

console.log('```');

console.log('\n🎯 Summary:');
console.log('===========');
console.log(`- Total schemas: ${schemaFiles.length}`);
console.log(`- Schemas with dependencies: ${Object.values(dependencies).filter(refs => refs.length > 0).length}`);
console.log(`- Self-contained schemas: ${Object.values(dependencies).filter(refs => refs.length === 0).length}`);
console.log(`- Unique references: ${allRefs.size}`);

// Check for missing dependencies
console.log('\n⚠️  Missing Dependencies:');
console.log('========================');
let missingCount = 0;

for (const [schema, refs] of Object.entries(dependencies)) {
  refs.forEach(ref => {
    if (ref.startsWith('./') || ref.startsWith('../')) {
      const refPath = path.resolve(path.dirname(path.join(schemasDir, schema)), ref);
      if (!fs.existsSync(refPath)) {
        console.log(`❌ ${schema} → ${ref} (file not found)`);
        missingCount++;
      }
    }
  });
}

if (missingCount === 0) {
  console.log('✅ No missing dependencies found!');
} else {
  console.log(`❌ Found ${missingCount} missing dependencies`);
}

console.log('\n🚀 Schema graph analysis complete!'); 