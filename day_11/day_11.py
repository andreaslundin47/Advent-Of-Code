with open('input', 'r') as f :
    grid = [[int(v) for v in line.strip()] for line in f.readlines()]

octos = {(r,c):v for r, row in enumerate(grid) for c,v in enumerate(row)}
flash_count = 0
step = 0

while True :
    step += 1

    # Increment all to begin with
    for pos in octos.keys() :
        octos[pos] += 1

    # Loop while we still have octopuses that should, but haven't flashed yet
    flashed = set()
    about_to_flash = [pos for pos,level in octos.items() if level > 9 and pos not in flashed]
    while about_to_flash :
        for r,c in about_to_flash :
            flashed.add( (r,c) )
            neighbours = [(r+i,c+j) for i in range(-1,2) for j in range(-1,2) if not (i==0 and j==0)]
            for neighbour in neighbours :
                if neighbour in octos :
                    octos[neighbour] += 1
        about_to_flash = [pos for pos,level in octos.items() if level > 9 and pos not in flashed]

    # For first 100 steps add number of flashes to the tally
    if step <= 100 :
        flash_count += len(flashed)
    
    # Stop on the step when every single octopus flashed during the step
    if len(flashed) == len(octos) :
        break

    # Reset level of all that flashed during this step
    for pos,level in octos.items() :
        if level > 9 :
            octos[pos] = 0

print(f"Part 1. Number of flashed in first 100 steps: {flash_count}")
print(f"Part 2. Step on wich all octopuses flashed:   {step}")