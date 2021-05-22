from functools import reduce

with open('input', 'r') as f:
    earliest_ts = int(f.readline())
    all_buses = [b for b in f.readline().strip().split(',')]

# Part 1
buses = [int(b) for b in all_buses if b != 'x']
multiples = [earliest_ts//b for b in buses]
relative_departures = [(m+1)*b - earliest_ts for m, b in zip(multiples, buses)]
departure, bus_id = min(zip(relative_departures, buses), key=lambda p: p[0])

answer = departure * bus_id
print(f"Part 1. Mult: {answer}")


# Part 2

def dio(a,b,c):
    q,r = divmod(a,b)
    if r == 0:
        return (0, c//b)
    else:
        u,v = dio(b,r,c)
        return (v, u-q*v)

def CRT(divisors, remainders):
    N = reduce(lambda x,y: x*y, divisors)
    Nis = [N // n for n in divisors]
    xis = [dio(Ni,-n,1)[0] for Ni, n in zip(Nis, divisors)]
    t = sum(x*Ni*b for x, Ni, b in zip(xis, Nis, remainders)) % N
    return t

buses_shifts = [(int(b), s) for s,b in enumerate(all_buses) if b != 'x']
mods, shifts = zip(*buses_shifts)
rems = [(-s % p) for s,p in zip(shifts, mods)]
t = CRT(mods, rems)
print(f"Part 2. Smallest t is {t}")