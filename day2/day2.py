with open("day2/input.txt") as f:
    lines = f.readlines()

# contents = {"red": 12, "green": 13, "blue": 14}

total = 0
for line in lines:
    # invalid_game = False
    minimum = {"red": 0, "green": 0, "blue": 0}
    line = line.strip()
    game_id, game = line.split(": ")
    draws = game.split("; ")

    for draw in draws:
        colors = draw.split(", ")
        for color in colors:
            num, name = color.split(" ")
            if int(num) > minimum[name]:
                minimum[name] = int(num)
            # if int(num) > contents[name]:
            #     invalid_game = True
    power = minimum["red"] * minimum["green"] * minimum["blue"]
    total += power
    # if invalid_game:
    #     continue
    # total += int(game_id.split(" ")[1])


print(total)