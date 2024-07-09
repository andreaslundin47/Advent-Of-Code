with open('input', 'r') as f:
    moves = []
    for line in f.readlines() :
        d, v = line.strip().split()
        moves.append( (d, int(v)) )

# Part 1
horizontal , vertical = 0, 0

for di, le in moves :
    if   di == 'forward' :
        horizontal += le
    elif di == 'up' :
        vertical -= le
    elif di == 'down' :
        vertical += le

score = horizontal * vertical

print(f"Part 1. Score: {score}")


# Part 2
position , depth, aim = 0, 0, 0

for di, le in moves :
    if   di == 'forward' :
        position += le
        depth += aim * le
    elif di == 'up' :
        aim -= le
    elif di == 'down' :
        aim += le

score = position * depth

print(f"Part 2. Score: {score}")