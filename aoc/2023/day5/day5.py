import datetime
import sys

def pairwise(iterable):
    "s -> (s0, s1), (s2, s3), (s4, s5), ..."
    a = iter(iterable)
    return zip(a, a)


def day1():
    with open(sys.argv[1]) as f:
        lines = list(f.readlines())

        seeds = [int(x) for x in lines[0].split(": ")[1].split()]

        categories = []
        for line in lines[2:]:
            if line[0].isalpha():
                categories.append([])
            elif line[0].isdigit():
                mapping = [int(x) for x in line.strip().split()]
                categories[-1].append(mapping)
            elif line.strip() == "":
                categories[-1].sort(key=lambda x: x[1])

        final_look_for = sys.maxsize
        for seed in seeds:
            looking_for = seed
            for i, category in enumerate(categories):
                for j, mapping in enumerate(category):
                    dest, source, rangee = mapping

                    if looking_for >= source and looking_for <= (source + rangee - 1):
                        diff = looking_for - source

                        to = dest + diff
                        looking_for = to
                        break
            final_look_for = min(looking_for, final_look_for)
        print(final_look_for)

def day2():
    with open(sys.argv[1]) as f:
        lines = list(f.readlines())

        seeds = [int(x) for x in lines[0].split(": ")[1].split()]

        categories = []
        for line in lines[2:]:
            if line[0].isalpha():
                categories.append([])
            elif line[0].isdigit():
                mapping = [int(x) for x in line.strip().split()]
                categories[-1].append(mapping)
            elif line.strip() == "":
                categories[-1].sort(key=lambda x: x[1])

        final_look_for = sys.maxsize
        for seed_start, seed_range in pairwise(seeds):
            print("[{}] Seed: {}".format(datetime.datetime.now(), seed_start))
            for seed in range(seed_start, seed_start + seed_range):
                looking_for = seed
                for i, category in enumerate(categories):
                    for j, mapping in enumerate(category):
                        dest, source, rangee = mapping

                        if looking_for >= source and looking_for <= (source + rangee - 1):
                            diff = looking_for - source

                            to = dest + diff
                            looking_for = to
                            break
                final_look_for = min(looking_for, final_look_for)
        print(final_look_for)


"""
Found 13 in [39, 0, 15]
0->39
1->40
2->41
3->43
4->44
5->45
6->46
7->47
8->48
9->49
10->50
11->51
12->52
13->53
"""

day1()
day2()
