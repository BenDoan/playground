import sys
import re

def get_nums(line):
    return [int(x) for x in line.split(":")[1].strip().split()]

def day1():
    with open(sys.argv[1]) as f:
        lines = f.readlines()

        times = get_nums(lines[0])
        dists = get_nums(lines[1])

        ways_product = 1
        for time, dist in zip(times, dists):
            ways = 0
            for second in range(1, time):
                left = time - second

                millis = left * second

                if millis > dist:
                    ways += 1
            ways_product *= ways
    print(ways_product)

def day2():
    with open(sys.argv[1]) as f:
        lines = f.readlines()

        times = get_nums(lines[0])
        dists = get_nums(lines[1])

        time = int("".join(map(str, times)))
        dist = int("".join(map(str, dists)))

        ways = 0
        for second in range(1, time):
            left = time - second

            millis = left * second

            if millis > dist:
                ways += 1
        print(ways)

day1()
day2()
