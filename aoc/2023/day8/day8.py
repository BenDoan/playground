import sys
import math

def part1():
    with open(sys.argv[1]) as f:
        lines = [x.strip() for x in f.readlines()]

        instructions = list(lines[0])

        mapping = {}
        for line in lines[2:]:
            origin, deststr = line.split(" = ")
            left, right = deststr.split(", ")
            leftnode = left.replace("(", "")
            rightnode = right.replace(")", "")
            mapping[origin] = (leftnode, rightnode)
        print(mapping)

        curr = "AAA"
        steps = 0
        while True:
            for i, instruction in enumerate(instructions):
                steps += 1
                node = mapping[curr]
                if instruction == "R":
                    curr = node[1]
                elif instruction == "L":
                    curr = node[0]

                print(f"c: {curr}, chose: {instruction}")
                if curr == "ZZZ":
                    print(steps)
                    return

def part2():
    with open(sys.argv[1]) as f:
        lines = [x.strip() for x in f.readlines()]

        instructions = list(lines[0])

        mapping = {}
        for line in lines[2:]:
            origin, deststr = line.split(" = ")
            left, right = deststr.split(", ")
            leftnode = left.replace("(", "")
            rightnode = right.replace(")", "")
            mapping[origin] = (leftnode, rightnode)

        currs = [x for x in mapping.keys() if x.endswith("A")]
        start_to_steps = {}
        steps = 0
        while True:
            for i, instruction in enumerate(instructions):
                steps += 1
                for j, curr in enumerate(currs):
                    node = mapping[curr]
                    if instruction == "R":
                        currs[j] = node[1]
                    elif instruction == "L":
                        currs[j] = node[0]

                for j, curr in enumerate(currs):
                    if curr.endswith("Z"):
                        start_to_steps[j] = steps
                        if len(start_to_steps) == len(currs):
                            print(math.lcm(*start_to_steps.values()))
                            return


#part1()
part2()
