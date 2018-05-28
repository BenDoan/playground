#!/usr/bin/env python

# http://www.pythonchallenge.com/pc/def/map.html

def translate(inp):
    s = []
    for c in inp:
        if not c.isalpha():
            tc = c
        elif c == 'y':
            tc = 'a'
        elif c =='z':
            tc = 'b'
        else:
            tc = chr(ord(c) + 2)
        s.append(tc)
    return "".join(s)

inp = """g fmnc wms bgblr rpylqjyrc gr zw fylb. rfyrq ufyr amknsrcpq ypc dmp. bmgle gr gl zw fylb gq glcddgagclr ylb rfyr'q ufw rfgq rcvr gq qm jmle. sqgle qrpgle.kyicrpylq() gq pcamkkclbcb. lmu ynnjw ml rfc spj."""


print(translate(inp))
print(translate("map"))
