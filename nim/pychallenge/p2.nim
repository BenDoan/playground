import strutils

proc translate_string(str: string, shift: int) =
    for i in toUpper(str):
        if ord(i) > 64 and ord(i) < 91:
            stdout.write chr((ord(i) - 65 + 2) mod 26+65)
        else:
            stdout.write " "
    echo ""


let input = "g fmnc wms bgblr rpylqjyrc gr zw fylb. rfyrq ufyr amknsrcpq ypc dmp. bmgle gr gl zw fylb gq glcddgagclr ylb rfyr'q ufw rfgq rcvr gq qm jmle. sqgle qrpgle.kyicrpylq() gq pcamkkclbcb. lmu ynnjw ml rfc spj."

translate_string(input, 3)

translate_string("map", 3)
