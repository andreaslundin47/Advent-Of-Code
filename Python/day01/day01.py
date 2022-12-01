with open('day01.txt', 'r') as f :
    inventories = f.read().split("\n\n")

caloriesPerElf = []
for inventory in inventories :
    inventoryContents = inventory.strip().split("\n")
    calories = sum( int(item) for item in inventoryContents )
    caloriesPerElf.append(calories)

print(f"Part 1. {max(caloriesPerElf)}")
print(f"Part 2. {sum(sorted(caloriesPerElf)[-3:])}")