import sys
from functools import cmp_to_key
from collections import defaultdict
from enum import Enum

CARD_VALS = {
 "A":14,
 "K":13,
 "Q":12,
 "J":11,
 "T":10,
 "9":9,
 "8":8,
 "7":7,
 "6":6,
 "5":5,
 "4":4,
 "3":3,
 "2":2,
 "1":1,
 }


class HandType(Enum):
    FiveOfAKind = 7
    FourOfAKind = 6
    FullHouse = 5
    ThreeOfAKind = 4
    TwoPair = 3
    OnePair = 2
    HighCard = 1


def group_count(counts, num):
    return len([x for x in counts.values() if x == num])

def hand_type(hand):
    counts = defaultdict(int)
    for card in hand:
        counts[card] += 1

    if group_count(counts, 5) == 1:
        return HandType.FiveOfAKind
    elif group_count(counts, 4) == 1:
        return HandType.FourOfAKind
    elif group_count(counts, 3) == 1 and group_count(counts, 2) == 1:
        return HandType.FullHouse
    elif group_count(counts, 3) == 1:
        return HandType.ThreeOfAKind
    elif group_count(counts, 2) == 2:
        return HandType.TwoPair
    elif group_count(counts, 2) == 1:
        return HandType.OnePair
    else:
        return HandType.HighCard
    return 0


def compare_hands(hand1, hand2):
    type1 = hand_type(hand1[0])
    type2 = hand_type(hand2[0])
    if type1.value > type2.value:
        return -1
    elif type1.value < type2.value:
        return 1
    else:
        order = sorted([hand1[0], hand2[0]], key=lambda x: [CARD_VALS[y] for y in x])
        if hand1[0] == order[0]:
            return 1
        else:
            return -1

def part1():
    with open(sys.argv[1]) as f:
        hands = []
        for line in f:
            hand, bid = line.strip().split()
            hands.append((hand, bid))

        hands.sort(key=cmp_to_key(compare_hands), reverse=True)

        s = 0
        for i, hand in enumerate(hands):
            s += int(hand[1]) * (i+1)
        print(s)


part1()
