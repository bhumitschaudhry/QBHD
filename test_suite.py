#!/usr/bin/env python3
"""
QBHD Test Suite
Automated testing for compiler functionality
"""

import subprocess
import sys
import json
import os

class Colors:
    GREEN = '\033[92m'
    RED = '\033[91m'
    YELLOW = '\033[93m'
    BLUE = '\033[94m'
    END = '\033[0m'

def run_cmd(cmd):
    """Run command and return exit code, stdout, stderr"""
    result = subprocess.run(cmd, shell=True, capture_output=True, text=True)
    return result.returncode, result.stdout, result.stderr

def test_version():
    """Test --version flag"""
    code, out, _ = run_cmd("./qbhd --version")
    return code == 0 and "QBHD" in out

def test_help():
    """Test --help flag"""
    code, out, _ = run_cmd("./qbhd --help")
    return code == 0 and "--json" in out

def test_check():
    """Test --check flag"""
    code, _, _ = run_cmd("./qbhd --check test_hello.bas")
    return code == 0

def test_json():
    """Test --json flag"""
    code, out, _ = run_cmd("./qbhd --json --check test_hello.bas")
    if code != 0:
        return False
    try:
        json.loads(out) if out.strip() else []
        return True
    except:
        return False

def test_compile():
    """Test basic compilation"""
    code, _, _ = run_cmd("./qbhd -x test_hello.bas")
    return code == 0

def test_optimize():
    """Test --optimize flag"""
    code, _, _ = run_cmd("./qbhd --optimize 2 -x test_hello.bas")
    return code == 0

def test_output():
    """Test --output flag"""
    code, _, _ = run_cmd("./qbhd --output test_out test_hello.bas")
    exists = os.path.exists("./test_out")
    if exists:
        os.remove("./test_out")
    return code == 0 and exists

def run_tests():
    """Run all tests"""
    tests = [
        ("Version flag", test_version),
        ("Help flag", test_help),
        ("Check flag", test_check),
        ("JSON output", test_json),
        ("Basic compile", test_compile),
        ("Optimize flag", test_optimize),
        ("Output flag", test_output),
    ]
    
    print(f"\n{Colors.BLUE}=== QBHD Test Suite ==={Colors.END}\n")
    
    passed = 0
    failed = 0
    
    for name, test_func in tests:
        try:
            result = test_func()
            if result:
                print(f"{Colors.GREEN}✓{Colors.END} {name}")
                passed += 1
            else:
                print(f"{Colors.RED}✗{Colors.END} {name}")
                failed += 1
        except Exception as e:
            print(f"{Colors.RED}✗{Colors.END} {name} (exception: {e})")
            failed += 1
    
    print(f"\n{Colors.BLUE}Results:{Colors.END} {passed} passed, {failed} failed\n")
    
    return failed == 0

if __name__ == "__main__":
    if not os.path.exists("./qbhd"):
        print(f"{Colors.RED}Error: ./qbhd not found{Colors.END}")
        print("Run ./build.sh first")
        sys.exit(1)
    
    success = run_tests()
    sys.exit(0 if success else 1)
