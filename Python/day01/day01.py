with open('day01.txt', 'r') as f :
    rawInput = f.read()

caloriesPerElf = []

inventories = rawInput.strip().split("\n\n")

for inventory in inventories :
    contents = inventory.strip().split("\n")
    calories = sum( int(item) for item in contents )
    caloriesPerElf.append(calories)

highest = max(caloriesPerElf)
sumTopThree = sum(sorted(caloriesPerElf, reverse=True)[:3])

print(f"Part 1. {highest}")
print(f"Part 2. {sumTopThree}")