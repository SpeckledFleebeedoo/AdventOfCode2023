import datetime
import os

# day = datetime.date.today().day
day = input("Day number: ")
foldername = f"day{day}"
pyfilename = foldername + ".py"
os.mkdir(foldername)

with open(foldername + "/" + pyfilename, "w+") as f:
    pass
with open(foldername + "/input.txt", "w+") as f:
    pass