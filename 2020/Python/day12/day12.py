import math

with open('input', 'r') as f:
    parsed = [line.strip() for line in f.readlines()]
    moves = [(ac[0], int(ac[1:])) for ac in parsed]

def turn(curr, lr, angle):
    dirs = ['N', 'E', 'S', 'W'] 
    idx = dirs.index(curr)
    qts = int(angle / 90)
    idx = (idx-qts) % 4 if lr == 'L' else (idx+qts) % 4
    return dirs[idx]

def move(direction, dist, pos):
    x,y = pos
    if direction == 'N':
        return (x,y+dist) 
    elif direction == 'S':
        return (x,y-dist)
    elif direction == 'E':
        return (x+dist,y)
    elif direction == 'W':
        return (x-dist,y) 

def rotate(pos, lr, angle):
    x,y = pos
    if lr == 'R':
        angle = -angle
    rs = math.radians(angle)
    c, s = int(math.cos(rs)), int(math.sin(rs))
    x, y = x*c - y*s, x*s + y*c
    return (x,y)

# Part 1
pos = (0,0)
facing = 'E'
for action, argument in moves:
    if action in ['N', 'E', 'S', 'W']:
        pos = move(action, argument, pos)
    elif action == 'F':
        pos = move(facing, argument, pos)
    elif action in ('L', 'R'):
        facing = turn(facing, action, argument)
    print(f"{action}{argument:3d}, D={facing}, (x,y)=({pos[0]:4d},{pos[1]:4d})")
m = abs(pos[0]) + abs(pos[1])
print(f"Part 1. Manhattan distanstance: {m}")

# Part 2
ship = (0,0)
waypoint = (10,1)
for action, argument in moves:
    if action in ['N', 'E', 'S', 'W']:
        waypoint = move(action, argument, waypoint)
    elif action == 'F':
        ship = (ship[0] + waypoint[0] * argument, ship[1] + waypoint[1] * argument)
    elif action in ('L', 'R'):
        waypoint = rotate(waypoint, action, argument)
    print(f"{action}{argument:3d}, ship=({ship[0]:4d},{ship[1]:4d}), waypoint=({waypoint[0]:4d},{waypoint[1]:4d})")
m = abs(ship[0]) + abs(ship[1])
print(f"Part 2. Manhattan distanstance: {m}")

