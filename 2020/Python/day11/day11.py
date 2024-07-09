# Parse file
with open('input', 'r') as f:
    inps = [[c for c in line.strip()] for line in f.readlines()]

# Add padding
w, h = len(inps[0]), len(inps)
padded_seats = [['P' for x in range(w+2)] for y in range(h+2)]
for y in range(h):
    for x in range(w):
        padded_seats[y+1][x+1] = inps[y][x]

def draw(seats):
    for row in seats:
        print(''.join(row))
    print()

def neigh8(x, y):
    return [(xx,yy) for xx in range(x-1, x+2) for yy in range(y-1, y+2) if xx != x or yy != y]

def iterate(seating):
    w, h = len(seating[0]), len(seating)
    updated = [['P' for x in range(w)] for y in range(h)]
    changes = 0

    for y in range(1, h-1):
        for x in range(1, w-1):
            counts = {'.':0, 'L':0, '#':0, 'P':0}
            for xx,yy in neigh8(x,y):
                counts[seating[yy][xx]] += 1
            if seating[y][x] == 'L' and counts['#'] == 0:
                updated[y][x] = '#'
                changes += 1
            elif seating[y][x] == '#' and counts['#'] >= 4:
                updated[y][x] = 'L'
                changes += 1
            else:
                updated[y][x] = seating[y][x]

    return (changes > 0), updated

def iterate_second(seating):
    w, h = len(seating[0]), len(seating)
    updated = [['P' for x in range(w)] for y in range(h)]
    changes = 0

    for y in range(1, h-1):
        for x in range(1, w-1):

            occupied = 0
            for delta in neigh8(0,0):
                if is_occupied_seat_in_direction(x, y, delta, seating):
                    occupied += 1

            if seating[y][x] == 'L' and occupied == 0:
                updated[y][x] = '#'
                changes += 1
            elif seating[y][x] == '#' and occupied >= 5:
                updated[y][x] = 'L'
                changes += 1
            else:
                updated[y][x] = seating[y][x]

    return (changes > 0), updated



def is_occupied_seat_in_direction(x, y, deltas, seatings):
    xx, yy = x, y
    while True:
        xx += deltas[0]
        yy += deltas[1]
        c = seatings[yy][xx]
        if c == 'P' or c == 'L':
            return False
        if c == '#':
            return True

def count_occupied(seatings):
    occ = 0
    for row in seatings:
        for c in row:
            if c == '#':
                occ += 1
    return occ

# Part 1
changed = True
seats_a = padded_seats.copy()
while changed:
    draw(seats_a)
    changed, seats_a = iterate(seats_a)
occ = count_occupied(seats_a)
print(f"Part 1. Occupied seats: {occ}")

# Part 2
changed = True
seats_a = padded_seats.copy()
while changed:
    draw(seats_a)
    changed, seats_a = iterate_second(seats_a)
occ = count_occupied(seats_a)
print(f"Part 2. Occupied seats: {occ}")
