import functools


with open('day03.txt', 'r') as f :
    rucksacks = f.readlines()


def itemPriority(item) :
    if item.isupper() :
        return ord(item) - ord('A') + 27
    else :
        return ord(item) - ord('a') + 1


def solvePartOne() :
    prioSum = 0
    for rucksack in rucksacks :
        size = len(rucksack) // 2
        comp1, comp2 = set(rucksack[:size]), set(rucksack[size:])
        commonItem = comp1.intersection(comp2).pop()
        prioSum += itemPriority(commonItem)

    print(f"Part 1. Sum = {prioSum}")


def solvePartTwo() :
    prioSum = 0

    groups = [ rucksacks[i:i+3] for i in range(0, len(rucksacks), 3) ]
    for group in groups :
        sackSets = [set(sack.strip()) for sack in group]
        commonItem = functools.reduce(lambda acc, sack: acc.intersection(sack), sackSets).pop()
        prioSum += itemPriority(commonItem)

    print(f"Part 2. Sum = {prioSum}")


solvePartOne()
solvePartTwo()