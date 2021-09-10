#!/usr/bin/env bash

# A test that always fails after echoing the current platform.
# This lets you see where the test ran.
echo "Platform is: $(uname -a)"
exit 1
