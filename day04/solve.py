def parse(lines):
    for line in lines:
        nums = line.split(":")[1][1::]
        winning = list(filter(None, nums.split("|")[0].split(" ")))
        ours = list(filter(None, nums.split("|")[1].split(" ")))
        yield (winning, ours)


def count_hits(card):
    winning, ours = card
    return sum(_ in winning for _ in ours)


def score_card(hits):
    if hits == 0:
        return 0
    return 2 ** (hits - 1)


with open("input") as f:
    lines = f.read().splitlines()

cards = list(parse(lines))

total = {i: 1 for i in range(len(cards))}
for num, count in total.items():
    for i in range(count):
        hits = count_hits(cards[num])
        for j in range(num + 1, num + hits + 1):
            total[j] += 1

score = sum(v for v in total.values())

print(score)
