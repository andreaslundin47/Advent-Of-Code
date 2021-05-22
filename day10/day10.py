with open('input', 'r') as f:
    plug_joltages = [int(line.strip()) for line in f.readlines()]

joltages = sorted(plug_joltages)
inlet = joltages[-1] + 3
jolts = joltages.copy()
jolts.insert(0, 0)
jolts.append(inlet)

# Part 1
d1 = sum([1 for jl, ju in zip(jolts[:-1], jolts[1:]) if ju-jl==1])
d3 = sum([1 for jl, ju in zip(jolts[:-1], jolts[1:]) if ju-jl==3])
print(f"Part 1. Diff-product: {d1*d3}")

# Part 2
plugs = reversed(jolts)
memo = {next(plugs): 1}
for jolt in plugs:
    connects_to = [jolt+offset for offset in range(1,4)]
    memo[jolt] = sum([memo[c] for c in connects_to if c in memo])
print(f"Part 2. Number of ways to make connection: {memo[0]}")