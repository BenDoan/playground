#!/usr/bin/env python

# http://www.pythonchallenge.com/pc/def/ocr.html

with open("input.txt") as f:
    s = "".join(f.readlines())

print("".join(filter(lambda c: c.isalpha(), s)))

