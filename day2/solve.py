def parse(lines: list[str]):
    # This is awful
    games = {}
    for line in lines:
        game_num = int(line.split(":")[0].split(" ")[1])
        rounds = [_.strip() for _ in line.split(":")[1].split(";")]
        game = []
        for round in rounds:
            draws = round.split(", ")
            test = []
            for draw in draws:
                num, color = draw.split(" ")
                test.append((int(num), color))
            game.append(test)
        games[game_num] = game
    return games


with open("input") as f:
    games = parse(f.read().splitlines())


def get_score():
    score = 0
    for id, game in games.items():
        bad = False
        for round in game:
            for num, color in round:
                match color:
                    case "red":
                        if num > 12:
                            bad = True
                    case "green":
                        if num > 13:
                            bad = True
                    case "blue":
                        if num > 14:
                            bad = True
        if bad:
            continue
        score += id
    return score


def get_power():
    score = 0
    for game in games.values():
        min_r = 0
        min_g = 0
        min_b = 0
        for round in game:
            for num, color in round:
                match color:
                    case "red":
                        min_r = num if num > min_r else min_r
                    case "green":
                        min_g = num if num > min_g else min_g
                    case "blue":
                        min_b = num if num > min_b else min_b
        score += min_r * min_g * min_b
    return score


print(get_power())
