from itertools import product

with open('input', 'r') as f :
    look_up = f.readline().strip()
    image = [row for row in f.read().strip().split('\n')]


lights = set()
for y, row in enumerate(image) :
    for x, sym in enumerate(row) :
        if sym == '#' :
            lights.add( (x,y) )


def print_grid(coords) :
    min_x = min(c[0] for c in coords)
    min_y = min(c[1] for c in coords)
    max_x = max(c[0] for c in coords)
    max_y = max(c[1] for c in coords)

    out = ""
    for y in range(min_y, max_y+1) :
        for x in range(min_x, max_x+1) :
            if (x,y) in coords :
                out += "#"
            else :
                out += " "
        out += "\n"
    print(out)


def enhance(coords, table, inf_lights=None) :
    if not inf_lights :
        inf_lights = set()

    min_x = min(c[0] for c in coords)
    min_y = min(c[1] for c in coords)
    max_x = max(c[0] for c in coords)
    max_y = max(c[1] for c in coords)

    new_image = set()

    for x, y in product(range(min_x-1, max_x+2), range(min_y-1, max_y+2)) :
        signature = ""
        for ry, rx in product(range(y-1,y+2), range(x-1,x+2)) :
            if (rx,ry) in coords or (rx,ry) in inf_lights :
                signature += "1"
            else :
                signature += "0"
        if table[int(signature, 2)] == '#' :
            new_image.add( (x,y) )


    if table[0] == '#' and not inf_lights :
        inf_lights = set()
       
        for x in range(min_x-3, max_x+4) :
            inf_lights.add( (x, min_y-3) )
            inf_lights.add( (x, min_y-2) )
            inf_lights.add( (x, max_y+2) )
            inf_lights.add( (x, max_y+3) )

        for y in range(min_y-3, max_y+4) :
            inf_lights.add( (min_x-3, y) )
            inf_lights.add( (min_x-2, y) )
            inf_lights.add( (max_x+2, y) )
            inf_lights.add( (max_x+3, y) )
    
    elif table[0] == '#' and inf_lights :
        inf_lights = None
    
    return new_image, inf_lights


#### Part 1

L = lights.copy()
L, B = enhance(L, look_up)
L, B = enhance(L, look_up, B)
print(f"Part 1. Number of lights: {len(L)}")

#### Part 2

L = lights.copy()
B = None
for _ in range(50) :
    L, B = enhance(L, look_up, B)
print(f"Part 2. Number of lights: {len(L)}")