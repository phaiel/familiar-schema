#!/usr/bin/env node

const https = require('https');

function searchNpm(query) {
  return new Promise((resolve, reject) => {
    const url = `https://registry.npmjs.org/-/v1/search?text=${encodeURIComponent(query)}&size=15`;
    
    https.get(url, (res) => {
      let data = '';
      res.on('data', chunk => data += chunk);
      res.on('end', () => {
        try {
          const result = JSON.parse(data);
          resolve(result.objects || []);
        } catch (e) {
          reject(e);
        }
      });
    }).on('error', reject);
  });
}

async function main() {
  const query = process.argv.slice(2).join(' ');
  
  if (!query) {
    console.log('🔍 Package Search Tool - Find the RIGHT package name!');
    console.log('Usage: pkg-search <search-term>');
    console.log('Example: pkg-search "json schema"');
    console.log('Example: pkg-search "quicktype"');
    process.exit(1);
  }

  try {
    console.log(`🔍 Searching for packages related to: "${query}"`);
    console.log('─'.repeat(50));
    
    const results = await searchNpm(query);
    
    if (results.length === 0) {
      console.log('❌ No packages found');
      console.log('💡 Try different search terms or check spelling');
      return;
    }

    console.log(`✅ Found ${results.length} packages:\n`);
    
    results.forEach((pkg, index) => {
      console.log(`${index + 1}. ${pkg.package.name}`);
      console.log(`   📦 Version: ${pkg.package.version}`);
      console.log(`   📄 ${pkg.package.description || 'No description'}`);
      console.log(`   ⭐ Weekly downloads: ${pkg.package.links?.npm ? 'Available on npm' : 'N/A'}`);
      console.log(`   💾 Install: npm install -g ${pkg.package.name}`);
      console.log('');
    });
    
    console.log('💡 Pro tip: Always search BEFORE installing to avoid 404 errors!');
    
  } catch (error) {
    console.error('❌ Error searching:', error.message);
    console.log('💡 Try checking your internet connection or try again later');
  }
}

main(); 