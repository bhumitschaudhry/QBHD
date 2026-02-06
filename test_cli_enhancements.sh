#!/bin/bash
# Test QBHD CLI enhancements
# Run after applying enhancements and rebuilding

echo "=== Testing QBHD CLI Enhancements ==="
echo

# Check if qbhd exists
if [ ! -f "./qbhd" ]; then
    echo "❌ Error: ./qbhd not found"
    echo "   Run ./build_qbhd.sh first"
    exit 1
fi

echo "✓ QBHD compiler found"
echo

# Test 1: --version
echo "Test 1: --version flag"
./qbhd --version
if [ $? -eq 0 ]; then
    echo "✓ PASS"
else
    echo "❌ FAIL"
fi
echo

# Test 2: --help
echo "Test 2: --help flag"
./qbhd --help | grep -q "json"
if [ $? -eq 0 ]; then
    echo "✓ PASS (--json found in help)"
else
    echo "❌ FAIL (--json not in help)"
fi
echo

# Test 3: --check (syntax check)
echo "Test 3: --check flag"
./qbhd --check test_hello.bas > /dev/null 2>&1
if [ $? -eq 0 ]; then
    echo "✓ PASS"
else
    echo "❌ FAIL"
fi
echo

# Test 4: --json --check
echo "Test 4: --json --check flags"
output=$(./qbhd --json --check test_hello.bas 2>&1)
if [[ $output == "["* ]]; then
    echo "✓ PASS (JSON output detected)"
else
    echo "⚠ WARNING (No JSON output, but may be expected if no errors)"
fi
echo

# Test 5: --verbose
echo "Test 5: --verbose flag"
./qbhd --verbose test_hello.bas > /dev/null 2>&1
if [ $? -eq 0 ]; then
    echo "✓ PASS"
else
    echo "❌ FAIL"
fi
echo

# Test 6: --output
echo "Test 6: --output flag"
./qbhd --output test_output test_hello.bas > /dev/null 2>&1
if [ -f "./test_output" ]; then
    echo "✓ PASS (output file created)"
    rm -f ./test_output
else
    echo "❌ FAIL (output file not created)"
fi
echo

# Test 7: --optimize
echo "Test 7: --optimize flag"
./qbhd --optimize 2 test_hello.bas > /dev/null 2>&1
if [ $? -eq 0 ]; then
    echo "✓ PASS"
else
    echo "❌ FAIL"
fi
echo

# Test 8: --debug
echo "Test 8: --debug flag"
./qbhd --debug test_hello.bas > /dev/null 2>&1
if [ $? -eq 0 ]; then
    echo "✓ PASS"
else
    echo "❌ FAIL"
fi
echo

echo "=== Test Summary ==="
echo "All basic CLI flags tested"
echo
echo "Next steps:"
echo "  1. Review test results above"
echo "  2. If all pass, proceed to Phase 1, Task 3"
echo "  3. If any fail, check apply_cli_enhancements.sh"
