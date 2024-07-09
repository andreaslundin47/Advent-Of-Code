from dataclasses import dataclass

# Tile values: White => False, Black => True

# x-axis: horizontal toward east
# y-axis: Diagonally toward north east

deltas = {'e':(1,0), 'se':(1,-1), 'sw':(0,-1), 'w':(-1,0), 'nw':(-1,1), 'ne':(0,1)}

@dataclass
class Path:
    path: [str]

    def parse(text):
        path = []
        i = 0
        while i < len(text):
            d = text[i]
            if d in ('e', 'w'):
                i = i + 1
            else:
                d = text[i:i+2]
                i = i + 2
            path.append(d)
        return Path(path=path)

def flippin(black_tiles: set([(int,int)])) -> set([(int,int)]):
    adjacents = {}
    for x,y in black_tiles:
        # The black tile itself is an adjacent tile to other tiles
        adjacents[(x,y)] = adjacents.get((x,y), 0)
        # All six neighbours of tile has one more black adjacent neighbour
        neighbours = [(x+dx, y+dy) for dx, dy in deltas.values()]
        for nx, ny in neighbours:
            adjacents[(nx, ny)] = adjacents.get((nx,ny), 0) + 1
    
    tiles = {   (x,y)
                for (x,y), blacks in adjacents.items()
                if ( (x,y) in black_tiles and 1 <= blacks <= 2 ) 
                    or ( (x,y) not in black_tiles and blacks == 2 )
            }
    return tiles



with open('input', 'r') as f:
    lines = [line.strip() for line in f.read().strip().split('\n')]
    paths = [Path.parse(line) for line in lines]


# Part 1
tiles = {(0,0):False}
for path in paths:
    x,y = 0,0 # Reset to origin for each path
    for move in path.path:
        dx, dy = deltas[move]
        x += dx; y += dy
    tiles[(x,y)] = not tiles.get((x,y), False)

blacks = len([tile for tile in tiles.values() if tile == True])
print(f"Part 1. Black tiles: {blacks}")

# Part 2
black_tiles = {xy for xy, color in tiles.items() if color == True}
for day in range(100):
    black_tiles = flippin(black_tiles)
print(f"Part 2. Black tiles on day {day+1:3}: {len(black_tiles)}")