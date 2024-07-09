import math

with open('input', 'r') as f :
    ranges = f.readline().strip().strip('target area: ')
    xr, yr = ranges.split(', ')
    xr = [int(x) for x in xr.strip('x=').split('..')]
    yr = [int(y) for y in yr.strip('y=').split('..')]

maximum_height = 0
valid_initial_velocities = set()

vx_min = int(math.floor(math.sqrt(min(xr)) - 1/2))
vx_max = max(xr)
x_min, x_max = min(xr), max(xr)
y_min, y_max = min(yr), max(yr)
time_limit = 2000

for vx in range(vx_min, vx_max+1) :
    v_at_t = vx
    x_at_t = 0
    for t in range(1, time_limit) :
        x_at_t = x_at_t + v_at_t
        v_at_t = max(v_at_t-1, 0)
        if x_min <= x_at_t <= x_max :
            for y in range(y_min, y_max+1) :
                vy_approx = (2*y + t*(t-1))//(2*t)
                vy_candidates = math.floor(vy_approx), math.ceil(vy_approx)
                for vy in vy_candidates :
                    y_exaxt = vy*t - t * (t-1) // 2
                    if y == y_exaxt :
                        valid_initial_velocities.add( (vx,vy) )
                        height = vy * (1 + vy) // 2
                        maximum_height = height if height > maximum_height else maximum_height

print(f"Part 1. Maximum height: {maximum_height}")
print(f"Part 2. Number of possible initial velocities: {len(valid_initial_velocities)}")