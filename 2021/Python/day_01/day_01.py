with open('input', 'r') as f :
    values = [int(line.strip()) for line in f.readlines()]

incs = sum(b > a for a, b in zip(values[:-1], values[1:]))
print(f"Part 1. Increases: {incs}")

incs = sum(v3 > v0 for v3, v0 in zip(values[3:], values[:-3]))
print(f"Part 2. Increases: {incs}")