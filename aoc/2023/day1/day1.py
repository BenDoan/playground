import sys
import re

mapping = {
    "one": 1,
    "two": 2,
    "three": 3,
    "four": 4,
    "five": 5,
    "six": 6,
    "seven": 7,
    "eight": 8,
    "nine": 9
}


pattern = r"(\d|one|two|three|four|five|six|seven|eight|nine)"
pattern_rev = r"(\d|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)"

with open(sys.argv[1]) as f:
    s = 0
    for line in f:
        first_str = re.findall(pattern, line)[0]
        if first_str[0].isdigit():
            first_num = int(first_str)
        else:
            first_num = mapping[first_str]

        last_str = re.findall(pattern_rev, line[::-1])[0]
        if last_str[0].isdigit():
            last_num = int(last_str)
        else:
            last_num = mapping[last_str[::-1]]

        s += int(f"{first_num}{last_num}")

    print(s)
