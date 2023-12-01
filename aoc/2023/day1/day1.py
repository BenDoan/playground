import sys
import re

mapping = {"one": 1, "two": 2, "three": 3, "four": 4, "five": 5, "six": 6, "seven": 7, "eight": 8, "nine": 9, "1": 1, "2": 2, "3": 3, "4": 4, "5": 5, "6": 6, "7": 7, "8": 8, "9": 9}
pattern = r"(\d|one|two|three|four|five|six|seven|eight|nine)"
pattern_rev = r"(\d|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)"

with open(sys.argv[1]) as f:
    s = 0
    for line in f:
        first = re.findall(pattern, line)[0]
        last = re.findall(pattern_rev, line[::-1])[0]
        s += int(f"{mapping[first]}{mapping[last[::-1]]}")

    print(s)
