import sys
import itertools

def part1():
    with open(sys.argv[1]) as f:
        s = 0
        for line in f:
            s += process1([int(x) for x in line.strip().split()])
        print(s)

def process1(line):
    steps = [line]
    while True:
        new_step = []
        for a, b in itertools.pairwise(steps[-1]):
            new_step.append(b-a)
        steps.append(new_step)
        if sum(new_step) == 0:
            break
    steps = list(reversed(steps))

    for i, step in enumerate(steps[1:]):
        step.append(step[-1] + steps[i][-1])
    return steps[-1][-1]

def part2():
    with open(sys.argv[1]) as f:
        s = 0
        for line in f:
            s += process2([int(x) for x in line.strip().split()])
        print(s)

def process2(line):
    steps = [line]
    while True:
        new_step = []
        for a, b in itertools.pairwise(steps[-1]):
            new_step.append(b-a)
        steps.append(new_step)
        if sum(new_step) == 0:
            break
    steps = list(reversed(steps))

    for i, step in enumerate(steps[1:]):
        step.insert(0, step[0] - steps[i][0])
    return steps[-1][0]


part1()
part2()
