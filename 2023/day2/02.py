from pathlib import Path

MAX_RED = 12
MAX_GREEN = 13
MAX_BLUE = 14

max_dict = {
    "red": MAX_RED,
    "green": MAX_GREEN,
    "blue": MAX_BLUE,
}


def is_legal(games):
    legal = True
    for game in games.split(";"):
        for pair in game.split(","):
            pair: str
            (count, color) = pair.strip().split(" ")
            count = int(count)
            if count > max_dict[color]:
                legal = False
    return legal


def get_maxes(games):
    maxes = dict()
    for game in games.split(";"):
        for pair in game.split(","):
            pair: str
            (count, color) = pair.strip().split(" ")
            count = int(count)
            if color in maxes:
                if maxes[color] < count:
                    maxes[color] = count
            else:
                maxes[color] = count
    return maxes


total = 0
with Path("input").open() as file:
    for i, line in enumerate(file):
        games = line.split(":")[1].strip()
        maxes = get_maxes(games)
        power = maxes["red"] * maxes["green"] * maxes["blue"]
        total += power

print(total)
