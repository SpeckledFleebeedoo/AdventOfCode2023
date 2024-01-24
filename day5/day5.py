with open("day5/input.txt", "r") as f:
    text = f.read()

strings = text.split("\n\n")
seeds = [int(s) for s in strings[0].split(" ")[1:]]
mapStrings = strings[1:]

maps = [[[int(t) for t in s.split(" ")] for s in mapStrings[i].splitlines()[1:]] for i in range(len(mapStrings))]

positions = []
for seed in seeds:
    for map in maps:
        for maprange in map:
            if seed in range(maprange[1], maprange[1]+maprange[2]):
                seed = maprange[0] + seed - maprange[1]
                break
    positions.append(seed)

print(min(positions))