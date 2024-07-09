from functools import reduce

with open('input', 'r') as f :
    height_map = [[int(h) for h in row.strip()] for row in f.readlines()]

rows, cols = len(height_map), len(height_map[0])


#### Part 1

risk_sum = 0
low_points = []

for r, row in enumerate(height_map) :
    for c, height in enumerate(row) :
        non_low = False
        for rr, cc in [(r-1,c), (r+1,c), (r,c-1), (r,c+1)] :
            if 0 <= rr < rows and 0 <= cc < cols :
                if height >= height_map[rr][cc] :
                    non_low = True
        if not non_low :
            risk_sum += (1 + height)
            low_points.append( (r,c) )

print(f"Part 1. Sum of low point risk levels: {risk_sum}")


#### Part 2

def find_basin_size(starting_position) :
    """ Using basic BFS to explore the basin """
    basin = set()
    frontier = [starting_position]
    while frontier :
        pr, pc = frontier.pop(0)
        basin.add( (pr,pc) )
        for rr, cc in [(pr-1,pc), (pr+1,pc), (pr,pc-1), (pr,pc+1)] :
            if 0 <= rr < rows and 0 <= cc < cols :
                if height_map[rr][cc] > height_map[pr][pc] and height_map[rr][cc] < 9 :
                    if (rr,cc) not in basin and (rr,cc) not in frontier :
                        frontier.append( (rr,cc) )
    return len(basin)


basin_sizes = []
for point in low_points :
    basin_sizes.append(find_basin_size(point))

top3 = sorted(basin_sizes, reverse=True)[:3]
score = reduce(lambda x,y: x*y, top3)

print(f"Part 2. Basin multiple: {score}")