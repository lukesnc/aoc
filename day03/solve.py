import string
import itertools

with open("input") as f:
    engine = f.read()

adjacent = list(itertools.product([0, -1, 1], repeat=2))

sum = 0
grid = engine.splitlines()
for line in range(len(grid)):
    for chr in range(len(grid[line])):
        curr = grid[line][chr]
        if curr not in string.punctuation or curr == ".":
            continue

        found = []
        for y, x in adjacent:
            if grid[line + y][chr + x].isdigit():
                found.append((line + y, chr + x))

        nums = []
        for y, x in found:
            cursor = x
            while grid[y][cursor].isdigit():
                cursor -= 1
            cursor += 1

            num = ""
            while grid[y][cursor].isdigit():
                num += grid[y][cursor]
                cursor += 1
                if cursor > 139:
                    break
            num = int(num)

            if num not in nums:
                nums.append(num)

        if len(nums) < 2:
            continue
        product = 1
        for num in nums:
            product *= num
        sum += product


print(sum)
