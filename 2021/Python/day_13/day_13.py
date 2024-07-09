with open('input', 'r') as f :
    raw_pairs, raw_folds = f.read().strip().split('\n\n')
    pairs = []
    for line in raw_pairs.split('\n') :
        a, b = line.split(',')
        pairs.append( (int(a),int(b)) )
    folds = []
    for line in raw_folds.split('\n') :
        axis, coord = line.strip('fold along ').split('=')
        folds.append( (axis, int(coord)) )


def make_fold(dots, axis='x', coord=0) :
    resulting_dots = set()
    for x,y in dots :
        if axis == 'x' :
            if x <= coord :
                resulting_dots.add( (x, y) )
            else :
                resulting_dots.add( (2*coord-x, y) )
        elif axis == 'y' :
            if y <= coord :
                resulting_dots.add( (x, y) )
            else :
                resulting_dots.add( (x, 2*coord-y) )
    return resulting_dots


#### Part 1
print(f"Part 1. Number of dots left: {len(make_fold(set(pairs), folds[0][0], folds[0][1]))}")

#### Part 2
dots = set(pairs)
for axis, coord in folds :
    dots = make_fold(dots, axis, coord)

max_x = max(dots, key=lambda d: d[0])[0]
max_y = max(dots, key=lambda d: d[1])[1]
canvas = [['X' if (x,y) in dots else ' ' for x in range(max_x+1)] for y in range(max_y+1)]

print(f"Part 2. Password is:")
print("\n".join("".join(row) for row in canvas))