import re

with open("day1/input.txt") as f:
    lines = f.readlines()

translations = {"one": "1",
                "two": "2",
                "three": "3",
                "four": "4",
                "five": "5",
                "six": "6",
                "seven": "7",
                "eight": "8",
                "nine": "9",
                "1": "1",
                "2": "2",
                "3": "3",
                "4": "4",
                "5": "5",
                "6": "6",
                "7": "7",
                "8": "8",
                "9": "9"
                }

total = 0
for line in lines:
    numbers = re.findall("(?=([0-9]|one|two|three|four|five|six|seven|eight|nine))", line)
    first = numbers[0]
    last = numbers[-1]
    n = int(translations[first] + translations[last])
    total += n
    pass
print(total)