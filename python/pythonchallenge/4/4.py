#!/usr/bin/env python

# http://www.pythonchallenge.com/pc/def/equality.html

import re

with open("input.txt") as f:
    s = "\n".join(f.readlines())

print("".join(re.findall(r'[A-Z]{3}([a-z])[A-Z]{3}', s)))
