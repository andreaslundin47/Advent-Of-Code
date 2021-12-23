from collections import defaultdict

with open('input', 'r') as f :
    cubes = []
    lines = f.read().strip().split('\n')
    for line in lines :
        bounds = []
        mode, parts = line.split()
        for axis in parts.split(',') :
            axis = axis.split('=')[1]
            coords = axis.split('..')
            bounds.append( (int(coords[0]), int(coords[1])) )
        cubes.append( (mode, tuple(bounds)) )


def overlaps(c1, c2) :
    for a1, a2 in zip(c1, c2) :
        if a1[1] < a2[0] or a1[0] > a2[1] :
            return False
    return True


def get_overlap_bounds(c1, c2) :
    bounds = []
    for a1, a2 in zip(c1, c2) :
        bounds.append( (max(a1[0], a2[0]), min(a1[1], a2[1])) )
    return tuple(bounds)


def volume(bounds) :
    vol = 1
    for l,u in bounds :
        vol *= (u-l+1)
    return vol


"""
    Finds the total number of lights via a naive implementation of 
    the set inclusion and exclusion principle.
"""
def calculate_lights(cubes) :
    boxes = defaultdict(int)
    unique_id = 0
    for mode, bounds in cubes :
        next_boxes = defaultdict(int)
        for (box, _), contribution in boxes.items() :
            if overlaps(bounds, box) :
                overlap = get_overlap_bounds(bounds, box)
                op_sign = -1 if contribution > 0 else 1
                next_boxes[(overlap, unique_id)] += op_sign * volume(overlap)
                unique_id += 1
            next_boxes[(box, unique_id)] += contribution
            unique_id += 1
        if mode == 'on' :
            next_boxes[(bounds, unique_id)] += volume(bounds)
        boxes = {b:c for b,c in next_boxes.items() if c != 0}
    return sum(boxes.values())


bounds_505050 = ( (-50,50), (-50,50), (-50,50) )
center_cubes = filter(lambda c: overlaps(c[1], bounds_505050), cubes)

print(f"Part 1. Sum: {calculate_lights(center_cubes)}")
print(f"Part 2. Sum: {calculate_lights(cubes)}")