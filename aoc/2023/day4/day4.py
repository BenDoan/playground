import sys
from collections import defaultdict

def day1():
    with open(sys.argv[1]) as f:
        s = 0
        for line in f:
            card_data, nums = line.strip().split(": ")

            winning_nums_s, numbers_have_s = nums.split(" | ")
            numbers_have = numbers_have_s.split()
            winning_nums = winning_nums_s.split()

            winning_nums = set([int(x) for x in winning_nums])
            numbers_have = [int(x) for x in numbers_have]

            matches = []
            for num in numbers_have:
                if num in winning_nums:
                    matches.append(num)

            amount = 0
            for i, match in enumerate(matches):
                if i == 0:
                    amount += 1
                else:
                    amount *= 2
            s += amount
    print(s)
def day2():
    with open(sys.argv[1]) as f:
        s = 0
        lines = f.readlines()
        card_to_copies = [1] * (len(lines) + 1)
        card_to_copies[0] = 0
        for line_idx, line in enumerate(lines):
            card_num = line_idx + 1
            card_data, nums = line.strip().split(": ")

            winning_nums_s, numbers_have_s = nums.split(" | ")
            numbers_have = numbers_have_s.split()
            winning_nums = winning_nums_s.split()

            winning_nums = set([int(x) for x in winning_nums])
            numbers_have = [int(x) for x in numbers_have]

            matches = []
            for num in numbers_have:
                if num in winning_nums:
                    matches.append(num)

            for i, num in enumerate(matches):
                card_to_copies[card_num + i + 1] += card_to_copies[card_num]

        print(sum(card_to_copies))
day1()
day2()


"""
1 [83, 86, 17, 48]

2 [61, 32]

3 [21, 1]

4 [84]

5 []

6 []
"""
