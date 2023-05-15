#!/usr/bin/env python3

"""
Takes a list of lines and copy-pastes them for the errors.rs num.
Each line should be formatted like this:
    const cooldownConflictError = 4000
    const waypointNoAccessError = 4001
"""

import re
import sys

print("please copy-paste the error codes", file=sys.stderr)
print("and close stdin when you're done", file=sys.stderr)
print()

text = sys.stdin.read()
print()

pattern = re.compile(r"const (\w+) = (\d+)")
for line in text.splitlines():
    match = re.match(pattern, line.strip())
    if not match:
        continue
    name, i = match[1], match[2]
    print(i, "=>", f'"{name}",')
