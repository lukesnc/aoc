words = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]


def last(line: str):
    line = line[::-1]
    for i, chr in enumerate(line):
        if chr.isdigit():
            return int(chr)

        substr = line[i : i + 5]
        if len(substr) > 1 and substr[1].isdigit():
            return int(substr[1])

        for j, word in enumerate(
            [word[::-1] for word in words],
            start=1,
        ):
            if word in substr:
                return j
    return None


def first(line: str):
    for i, chr in enumerate(line):
        if chr.isdigit():
            return int(chr)

        substr = line[i : i + 5]
        if len(substr) > 1 and substr[1].isdigit():
            return int(substr[1])

        for j, word in enumerate(
            words,
            start=1,
        ):
            if word in substr:
                return j
    return None


with open("input") as f:
    lines = f.read().splitlines()

sum = 0
for line in lines:
    print(line)
    print(str(first(line)) + str(last(line)))
    sum += int(str(first(line)) + str(last(line)))

print(sum)
