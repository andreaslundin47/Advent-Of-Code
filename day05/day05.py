with open('input', 'r') as f:
    passes = [li.strip() for li in f.readlines()]

def bin2col(bin_str):
    return int(bin_str.replace('L', '0').replace('R', '1'), base=2)

def bin2row(bin_str):
    return int(bin_str.replace('F', '0').replace('B', '1'), base=2)

def pass2Id(bpass):
    row, seat =  bin2row(bpass[:-3]), bin2col(bpass[-3:])
    return row * 8 + seat

# Part 1
topID = max([pass2Id(p) for p in passes])
print(f"Part 1. Highest ID number: {topID}")

# Part 2
ids = [pass2Id(p) for p in passes]
missing = [pos for pos in range(127*8+8) if pos not in ids]

for m in missing:
    if m-1 in ids and m+1 in ids:
        myId = m
        break

print(myId)