import sys
from functools import cmp_to_key
from collections import defaultdict
from enum import Enum

CARD_VALS = {
 "A":14,
 "K":13,
 "Q":12,
 "T":10,
 "9":9,
 "8":8,
 "7":7,
 "6":6,
 "5":5,
 "4":4,
 "3":3,
 "2":2,
 "J":1,
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

def test_scoring():
    assert hand_type("22222") == HandType.FiveOfAKind
    assert hand_type("2J222") == HandType.FiveOfAKind
    assert hand_type("2JJJJ") == HandType.FiveOfAKind

    assert hand_type("23333") == HandType.FourOfAKind
    assert hand_type("J3332") == HandType.FourOfAKind
    assert hand_type("JJJ32") == HandType.FourOfAKind

    assert hand_type("33322") == HandType.FullHouse
    assert hand_type("2233J") == HandType.FullHouse

    assert hand_type("33324") == HandType.ThreeOfAKind
    assert hand_type("2344J") == HandType.ThreeOfAKind

    assert hand_type("22455") == HandType.TwoPair

    assert hand_type("22456") == HandType.OnePair
    assert hand_type("2345J") == HandType.OnePair

    assert hand_type("12345") == HandType.HighCard

def hand_type(hand):
    counts = defaultdict(int)
    for card in hand:
        counts[card] += 1

    has_joker = "J" in hand

    max_count = 0
    max_card = "2"
    for card, count in counts.items():
        if card == "J":
            continue
        if count > max_count:
            max_count = count
            max_card = card

    new_hand = hand.replace("J", max_card)
    counts = defaultdict(int)
    for card in new_hand:
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

def part2():
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


test_scoring()
part2()
