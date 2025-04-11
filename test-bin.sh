#!/bin/bash

echo "This is a test binary file"
echo "Current date and time: $(date)"
echo "Current directory: $(pwd)"
echo "Arguments received: $@"

# Generate some sample output
echo "System information:"
echo "-------------------"
uname -a
echo ""

echo "Memory usage:"
echo "-------------"
free -h
echo ""

echo "Disk usage:"
echo "-----------"
df -h
echo ""

# Exit with success
exit 0
