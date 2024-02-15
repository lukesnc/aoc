from __future__ import annotations
from dataclasses import dataclass
from functools import cmp_to_key


@dataclass
class Hand:
    cards: str
    winnings: int

    def count_cards(self) -> dict:
        return {c: self.cards.count(c) for c in self.cards if c != "J"}

    @property
    def value(self) -> int:
        counts = sorted(list(self.count_cards().values()), reverse=True) or [0]
        jokers = self.cards.count("J")
        if counts[0] + jokers == 5:
            return 7
        elif counts[0] + jokers == 4:
            return 6
        elif counts[0] + jokers == 3 and counts[1] == 2:
            return 5
        elif counts[0] + jokers == 3:
            return 4
        elif counts[0] == 2 and (jokers or counts[1] == 2):
            return 3
        elif counts[0] == 2 or jokers:
            return 2
        return 1

    def __gt__(self, rhs: Hand):
        if self.value == rhs.value:
            for self_card, rhs_card in zip(self.cards, rhs.cards):
                if card_value(self_card) == card_value(rhs_card):
                    continue
                if card_value(self_card) > card_value(rhs_card):
                    return True
                return False
        return self.value > rhs.value


def parse(lines: list[str]):
    for line in lines:
        yield Hand(line.split(" ")[0], int(line.split(" ")[1]))


def card_value(card: str) -> int:
    return "J23456789TQKA".index(card)


def compare(rhs: Hand, lhs: Hand):
    if rhs > lhs:
        return 1
    elif lhs > rhs:
        return -1
    return 0


with open("input") as f:
    data = list(parse(f.read().splitlines()))

data.sort(key=cmp_to_key(compare))
score = sum(i * hand.winnings for i, hand in enumerate(data, start=1))
print(score)
