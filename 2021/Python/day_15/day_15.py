from queue import PriorityQueue
from itertools import product

with open('input', 'r') as f :
    grid_one = [[int(v) for v in line.strip()] for line in f.readlines()]

def dijkstra_risk(grid) :
    length, width = len(grid), len(grid[0])
    start, target = (0,0), (length-1, width-1)

    queue = PriorityQueue()
    queue.put( (0, start) )

    visited = set()
    risk = { start : 0 }

    while queue.qsize() :
        current = queue.get()[1]
        visited.add(current)

        if current == target :
            break

        y,x = current
        for neighbour in [(y-1,x), (y+1,x), (y,x-1), (y,x+1)] :
            ny, nx = neighbour
            if 0 <= ny < length and 0 <= nx < width and neighbour not in visited :
                this_risk = risk[current] + grid[ny][nx]
                
                if neighbour not in risk or this_risk < risk[neighbour] :
                    risk[neighbour] = this_risk
                    queue.put( (this_risk, neighbour) )
    
    return risk[target]
                

#### Part 1
print(f"Part 1. Lowest risk total: {dijkstra_risk(grid_one)}")

#### Part 2

height, width = len(grid_one), len(grid_one[0])

grid_two = [[0 for x in range(5 * width)] for y in range(5 * height)]

for y,x in product(range(height), range(width)) :
    for i,j in product(range(5), repeat=2) :
        grid_two[y + i*height][x + j*width] = (grid_one[y][x]-1+i+j) % 9 + 1

# solve the same way as in part 1
print(f"Part 2. Lowest risk total: {dijkstra_risk(grid_two)}")