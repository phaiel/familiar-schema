#!/usr/bin/env python3
"""
Comprehensive Rust warning fixer for typify-generated code.

This tool ACTUALLY FIXES warnings by making code changes, not by suppressing them.
It addresses the specific patterns that cause warnings in generated code.
"""

import os
import re
import subprocess
import sys
from pathlib import Path
from typing import Dict, List, Set, Tuple
import json

class RustWarningFixer:
    """Fixes actual Rust warnings in generated code."""
    
    def __init__(self, project_path: Path):
        self.project_path = project_path
        self.src_path = project_path / "src"
        self.fixes_applied = 0
        
    def run_command(self, cmd: List[str]) -> Tuple[bool, str]:
        """Run a command and return success status and output."""
        try:
            result = subprocess.run(
                cmd, cwd=self.project_path, capture_output=True, text=True, timeout=300
            )
            return result.returncode == 0, result.stdout + result.stderr
        except Exception as e:
            return False, str(e)
    
    def get_clippy_warnings(self) -> List[Dict]:
        """Parse Clippy warnings into structured data."""
        success, output = self.run_command(["cargo", "clippy", "--message-format=json"])
        warnings = []
        
        for line in output.split('\n'):
            if line.strip():
                try:
                    data = json.loads(line)
                    if data.get('reason') == 'compiler-message':
                        message = data.get('message', {})
                        if message.get('level') == 'warning':
                            warnings.append(message)
                except json.JSONDecodeError:
                    continue
        
        return warnings
    
    def fix_ambiguous_glob_reexports(self) -> int:
        """Fix ambiguous glob re-exports by using explicit imports."""
        print("🔧 Fixing ambiguous glob re-exports...")
        fixes = 0
        
        # Find all mod.rs files
        mod_files = list(self.src_path.glob("**/mod.rs"))
        
        for mod_file in mod_files:
            try:
                with open(mod_file, 'r') as f:
                    content = f.read()
                
                original_content = content
                
                # Analyze what types are being re-exported from each module
                module_exports = self._analyze_module_exports(mod_file.parent)
                
                # Replace glob imports with explicit imports
                new_content = self._replace_glob_imports(content, module_exports)
                
                if new_content != original_content:
                    with open(mod_file, 'w') as f:
                        f.write(new_content)
                    fixes += 1
                    print(f"  ✅ Fixed {mod_file.relative_to(self.project_path)}")
                    
            except Exception as e:
                print(f"  ❌ Error processing {mod_file}: {e}")
        
        return fixes
    
    def _analyze_module_exports(self, module_dir: Path) -> Dict[str, Set[str]]:
        """Analyze what types each module exports."""
        exports = {}
        
        for rust_file in module_dir.glob("*.rs"):
            if rust_file.name == "mod.rs":
                continue
                
            module_name = rust_file.stem
            exports[module_name] = set()
            
            try:
                with open(rust_file, 'r') as f:
                    content = f.read()
                
                # Find public structs, enums, types
                pub_items = re.findall(r'pub\s+(?:struct|enum|type)\s+(\w+)', content)
                exports[module_name].update(pub_items)
                
                # Find pub use statements
                pub_uses = re.findall(r'pub\s+use\s+[^;]*::(\w+)', content)
                exports[module_name].update(pub_uses)
                
            except Exception:
                continue
        
        return exports
    
    def _replace_glob_imports(self, content: str, module_exports: Dict[str, Set[str]]) -> str:
        """Replace glob imports with explicit imports to avoid conflicts."""
        lines = content.split('\n')
        new_lines = []
        
        for line in lines:
            # Match pub use module::*;
            match = re.match(r'^pub use (\w+)::\*;$', line.strip())
            if match:
                module_name = match.group(1)
                if module_name in module_exports and module_exports[module_name]:
                    # Create explicit imports
                    exports = sorted(module_exports[module_name])
                    if len(exports) <= 10:  # Only if reasonable number
                        explicit_imports = ', '.join(exports)
                        new_line = f"pub use {module_name}::{{{explicit_imports}}};"
                        new_lines.append(new_line)
                        continue
            
            new_lines.append(line)
        
        return '\n'.join(new_lines)
    
    def fix_derivable_impls(self) -> int:
        """Fix derivable Default implementations."""
        print("🔧 Fixing derivable implementations...")
        fixes = 0
        
        rust_files = list(self.src_path.glob("**/*.rs"))
        
        for rust_file in rust_files:
            try:
                with open(rust_file, 'r') as f:
                    content = f.read()
                
                original_content = content
                
                # Fix struct Default implementations
                content = self._fix_struct_default_impls(content)
                
                # Fix enum Default implementations  
                content = self._fix_enum_default_impls(content)
                
                if content != original_content:
                    with open(rust_file, 'w') as f:
                        f.write(content)
                    fixes += 1
                    print(f"  ✅ Fixed {rust_file.relative_to(self.project_path)}")
                    
            except Exception as e:
                print(f"  ❌ Error processing {rust_file}: {e}")
        
        return fixes
    
    def _fix_struct_default_impls(self, content: str) -> str:
        """Convert manual struct Default impls to derive attributes."""
        # Pattern for struct Default implementations
        pattern = re.compile(
            r'impl\s+::std::default::Default\s+for\s+(\w+)\s*\{\s*'
            r'fn\s+default\(\)\s*->\s*Self\s*\{\s*'
            r'Self\s*\{\s*'
            r'((?:\s*\w+:\s*Default::default\(\),?\s*)*)'
            r'\}\s*'
            r'\}\s*'
            r'\}',
            re.MULTILINE | re.DOTALL
        )
        
        def replace_impl(match):
            struct_name = match.group(1)
            
            # Find the struct definition
            struct_pattern = re.compile(
                rf'(#\[derive\([^\]]*)\]\s*\n\s*(pub\s+struct\s+{struct_name})',
                re.MULTILINE
            )
            
            def add_default_derive(struct_match):
                derives = struct_match.group(1)
                struct_def = struct_match.group(2)
                
                if 'Default' not in derives:
                    new_derives = derives + ', Default'
                    return f"#[derive({new_derives})]\n{struct_def}"
                return struct_match.group(0)
            
            # Replace the struct definition
            content_with_derive = struct_pattern.sub(add_default_derive, content)
            
            # If no existing derive found, add new one
            if content_with_derive == content:
                simple_struct_pattern = re.compile(
                    rf'(pub\s+struct\s+{struct_name})',
                    re.MULTILINE
                )
                content_with_derive = simple_struct_pattern.sub(
                    rf'#[derive(Default)]\n\1',
                    content
                )
            
            return ""  # Remove the impl block
        
        return pattern.sub(replace_impl, content)
    
    def _fix_enum_default_impls(self, content: str) -> str:
        """Convert manual enum Default impls to derive attributes."""
        # Pattern for enum Default implementations
        pattern = re.compile(
            r'impl\s+::std::default::Default\s+for\s+(\w+)\s*\{\s*'
            r'fn\s+default\(\)\s*->\s*Self\s*\{\s*'
            r'\w+::\s*(\w+)\s*'
            r'\}\s*'
            r'\}',
            re.MULTILINE | re.DOTALL
        )
        
        def replace_impl(match):
            enum_name = match.group(1)
            default_variant = match.group(2)
            
            # Find enum definition and add derives
            enum_pattern = re.compile(
                rf'(#\[derive\([^\]]*)\]\s*\n\s*(pub\s+enum\s+{enum_name}.*?\{{)',
                re.MULTILINE | re.DOTALL
            )
            
            def add_default_derive(enum_match):
                derives = enum_match.group(1)
                enum_def = enum_match.group(2)
                
                if 'Default' not in derives:
                    new_derives = derives + ', Default'
                    return f"#[derive({new_derives})]\n{enum_def}"
                return enum_match.group(0)
            
            content_with_derive = enum_pattern.sub(add_default_derive, content)
            
            # Add #[default] to the variant
            variant_pattern = re.compile(rf'(\s+)({default_variant})(,?)')
            content_with_derive = variant_pattern.sub(
                rf'\1#[default]\n\1\2\3',
                content_with_derive
            )
            
            return ""  # Remove the impl block
        
        return pattern.sub(replace_impl, content)
    
    def fix_needless_borrows(self) -> int:
        """Fix needless borrows in pattern matching."""
        print("🔧 Fixing needless borrows...")
        fixes = 0
        
        rust_files = list(self.src_path.glob("**/*.rs"))
        
        for rust_file in rust_files:
            try:
                with open(rust_file, 'r') as f:
                    content = f.read()
                
                original_content = content
                
                # Fix (&*PATTERN) -> PATTERN
                content = re.sub(
                    r'\(\&\*PATTERN\)\.find\(value\)\.is_none\(\)',
                    'PATTERN.find(value).is_none()',
                    content
                )
                
                if content != original_content:
                    with open(rust_file, 'w') as f:
                        f.write(content)
                    fixes += 1
                    print(f"  ✅ Fixed {rust_file.relative_to(self.project_path)}")
                    
            except Exception as e:
                print(f"  ❌ Error processing {rust_file}: {e}")
        
        return fixes
    
    def fix_unused_imports(self) -> int:
        """Remove unused imports."""
        print("🔧 Fixing unused imports...")
        
        # Use cargo fix to handle unused imports
        success, output = self.run_command([
            "cargo", "fix", "--allow-dirty", "--allow-staged"
        ])
        
        if success:
            print("  ✅ Unused imports fixed by cargo fix")
            return 1
        else:
            print(f"  ⚠️  cargo fix issues: {output[:200]}...")
            return 0
    
    def fix_irrefutable_let_patterns(self) -> int:
        """Fix irrefutable let patterns."""
        print("🔧 Fixing irrefutable let patterns...")
        fixes = 0
        
        rust_files = list(self.src_path.glob("**/*.rs"))
        
        for rust_file in rust_files:
            try:
                with open(rust_file, 'r') as f:
                    content = f.read()
                
                original_content = content
                
                # Replace } else if let Ok(v) = value.parse() { with } else { let v = value.parse().unwrap(); 
                content = re.sub(
                    r'\}\s*else\s+if\s+let\s+Ok\(([^)]+)\)\s*=\s*([^{]+)\s*\{',
                    r'} else { let \1 = \2.unwrap();',
                    content
                )
                
                if content != original_content:
                    with open(rust_file, 'w') as f:
                        f.write(content)
                    fixes += 1
                    print(f"  ✅ Fixed {rust_file.relative_to(self.project_path)}")
                    
            except Exception as e:
                print(f"  ❌ Error processing {rust_file}: {e}")
        
        return fixes
    
    def verify_fixes(self) -> Tuple[bool, int]:
        """Verify that all warnings are fixed."""
        print("🔍 Verifying fixes...")
        
        # Check compilation
        success, output = self.run_command(["cargo", "check"])
        if not success:
            print(f"❌ Compilation failed: {output[:200]}...")
            return False, -1
        
        # Count remaining warnings
        success, output = self.run_command(["cargo", "clippy", "--quiet"])
        warning_count = output.count('warning:')
        
        print(f"📊 Remaining warnings: {warning_count}")
        return warning_count == 0, warning_count
    
    def fix_all_warnings(self) -> bool:
        """Fix all warnings in the generated code."""
        print(f"🔧 Fixing ALL Rust warnings in: {self.project_path}")
        print("=" * 60)
        
        total_fixes = 0
        
        # Step 1: Fix ambiguous glob re-exports
        total_fixes += self.fix_ambiguous_glob_reexports()
        
        # Step 2: Fix derivable implementations
        total_fixes += self.fix_derivable_impls()
        
        # Step 3: Fix needless borrows
        total_fixes += self.fix_needless_borrows()
        
        # Step 4: Fix unused imports
        total_fixes += self.fix_unused_imports()
        
        # Step 5: Fix irrefutable let patterns
        total_fixes += self.fix_irrefutable_let_patterns()
        
        # Step 6: Format code
        print("🎨 Formatting code...")
        self.run_command(["cargo", "fmt"])
        
        # Step 7: Verify all warnings are fixed
        all_fixed, remaining_warnings = self.verify_fixes()
        
        print("=" * 60)
        print(f"📊 Total fixes applied: {total_fixes}")
        
        if all_fixed:
            print("🎉 ALL WARNINGS FIXED! Code is now warning-free.")
            return True
        else:
            print(f"❌ {remaining_warnings} warnings remain - need additional fixes")
            return False

def main():
    """Main entry point."""
    if len(sys.argv) != 2:
        print("Usage: python fix_all_rust_warnings.py <rust_project_path>")
        sys.exit(1)
    
    project_path = Path(sys.argv[1])
    if not project_path.is_absolute():
        project_path = Path.cwd() / project_path
    
    if not project_path.exists() or not (project_path / "Cargo.toml").exists():
        print(f"❌ Invalid Rust project: {project_path}")
        sys.exit(1)
    
    fixer = RustWarningFixer(project_path)
    success = fixer.fix_all_warnings()
    
    if not success:
        print("\n❌ Failed to fix all warnings!")
        sys.exit(1)
    else:
        print("\n✅ All warnings successfully fixed!")

if __name__ == "__main__":
    main() 