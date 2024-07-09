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
    bounds = []
    for a1, a2 in zip(c1, c2) :
        if a1[1] < a2[0] or a1[0] > a2[1] :
            return None
        bounds.append( (max(a1[0], a2[0]), min(a1[1], a2[1])) )
    return tuple(bounds)


def volume(bounds) :
    vol = 1
    for l,u in bounds :
        vol *= (u-l+1)
    return vol


"""
    Finds the total number of lights using the set inclusion exclusion 
    principle. Inspired by William Y. Feng.
"""
def count_lights(cubes) :
    regions_and_contributions = defaultdict(int)

    for setting, new_cube_bounds in cubes :
        new_overlaps = defaultdict(int)

        for existing_region in regions_and_contributions.keys() :
            new_overlap = overlaps(new_cube_bounds, existing_region)
            if new_overlap :
                new_overlaps[new_overlap] -= regions_and_contributions[existing_region]

        if setting == 'on' :
            new_overlaps[new_cube_bounds] += 1

        for box, counts in new_overlaps.items() :
            regions_and_contributions[box] += counts

    return sum(volume(b) * count for b, count in regions_and_contributions.items())


bounds_505050 = ( (-50,50), (-50,50), (-50,50) )
center_cubes = filter(lambda c: overlaps(c[1], bounds_505050), cubes)

print(f"Part 1. Sum: {count_lights(center_cubes)}")
print(f"Part 2. Sum: {count_lights(cubes)}")