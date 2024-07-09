with open('input', 'r') as f:
    initial = [[c for c in line.strip()] for line in f.readlines()]

init3 =         {(x,y,0)
                    for y, row in enumerate(initial)
                    for x, c in enumerate(row)
                    if c == '#'
                }

init4 =         {(x,y,0,0)
                    for y, row in enumerate(initial)
                    for x, c in enumerate(row)
                    if c == '#'
                }

def neighbours26(coord):
    x,y,z = coord
    return {(x+i,y+j,z+k)
                for i in range(-1,2)
                for j in range(-1,2)
                for k in range(-1,2)
                if i!=0 or j!=0 or k!=0
            }

def neighbours80(coord):
    x,y,z,w = coord
    return {(x+i,y+j,z+k,w+l)
                for i in range(-1,2)
                for j in range(-1,2)
                for k in range(-1,2)
                for l in range(-1,2)
                if i!=0 or j!=0 or k!=0 or l!=0
            }


def step(active, neighbour_func):
    adjacents = {}
    for coord in active:
        # Include self
        adjacents[coord] = adjacents.get(coord, 0)
        # Include all 26 neighbours
        ns = neighbour_func(coord)
        for n in ns:
            adjacents[n] = adjacents.get(n, 0) + 1

    active_to_active = set()
    for a, n in adjacents.items():
        if (a in active) and (2 <= n <= 3):
            active_to_active.add(a)

    inactive_to_active = set()
    for a, n in adjacents.items():
        if (a not in active) and (n == 3):
            inactive_to_active.add(a)

    return active_to_active.union(inactive_to_active)


active = init3.copy()
for cycle in range(6):
    active = step(active, neighbours26)
print(f"Step 1. Active Cubes: {len(active)}")

active = init4.copy()
for cycle in range(6):
    active = step(active, neighbours80)
print(f"Step 2. Active Cubes: {len(active)}")