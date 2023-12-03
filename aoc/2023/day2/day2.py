import sys

def part1():
    with open(sys.argv[1]) as f:
        s = 0
        for i, line in enumerate(f):
            contents = line.split(":")[1]
            groups = contents.strip().split("; ")
            is_possible = True
            for group in groups:
                items = group.split(", ")
                for item in items:
                    num, color = item.split()
                    num = int(num)

                    if (color == "red" and num > 12) or (color == "green" and num > 13) or (color == "blue" and num > 14):
                        is_possible = False
            if is_possible:
                s += i+1
        print(s)

def part2():
    with open(sys.argv[1]) as f:
        s = 0
        for i, line in enumerate(f):
            max_blue = 0
            max_red = 0
            max_green = 0

            contents = line.split(":")[1]
            groups = contents.strip().split("; ")
            is_possible = True
            for group in groups:
                items = group.split(", ")
                for item in items:
                    num, color = item.split()
                    num = int(num)

                    if color == "blue":
                        max_blue = max(max_blue, num)
                    elif color == "red":
                        max_red = max(max_red, num)
                    elif color == "green":
                        max_green = max(max_green, num)

            product = max_blue * max_green * max_red
            s += product
        print(s)


part1()
part2()
