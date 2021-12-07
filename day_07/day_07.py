
with open('input', 'r') as f :
    positions = [int(v) for v in f.readline().strip().split(',')]


def cost_function_one(positions, x) :
    return sum(abs(p-x) for p in positions)


def cost_function_two(positions, x) :
    def cost(steps) :
        return steps * (1+steps) // 2
    return sum( cost(abs(p-x)) for p in positions )


def optimize(opt_func) :
    min_x, max_x = min(positions), max(positions)

    opt_cost = opt_func(positions, min_x)
    for x in range(min_x+1, max_x+1) :
        cost = opt_func(positions, x)
        if cost < opt_cost :
            opt_cost = cost

    return opt_cost

print(f"Part 1. Lowest fuel cost: {optimize(cost_function_one)}")
print(f"Part 2. Lowest fuel cost: {optimize(cost_function_two)}")