from functools import reduce

with open('input', 'r') as f:
    treeMap = [line.strip() for line in f.readlines()]


def trees_along(slope, tMap):
    h, w = len(tMap), len(tMap[0])
    dx, dy = slope

    ys = list(range(0, h, dy))
    trees = len([y for i, y in enumerate(ys) if tMap[y][(i*dx)%w] == '#'])
    return trees


# Part 1
t = trees_along((3,1), treeMap)
print(f"Part 1. Number of trees on path is {t}")

# Part 2
slopes = [(1,1), (3,1), (5,1), (7,1), (1,2)]
m = reduce(lambda x,y: x*y, [trees_along(slope, treeMap) for slope in slopes])
print(f"Part 2. Number of trees on path is {m}")