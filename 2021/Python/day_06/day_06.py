from collections import defaultdict

with open('input', 'r') as f :
    fish_numbers = [int(v) for v in f.readline().strip().split(',')]

def fish_after_days(initial_population, number_of_days) :
    state = defaultdict(lambda: 0)

    # Count the fish in the initial state
    for fish in initial_population :
        state[fish] += 1

    # For each new day, see how the counts depend on those of the previous day
    for day in range(number_of_days) :
        next_day = defaultdict(lambda: 0)
        for fish, count in state.items() :
            if fish == 0:
                next_day[6] += count
                next_day[8] += count
            else :
                next_day[fish-1] += count
        state = next_day
    
    return sum(state.values())


print(f"Part 1. After  80 days: {fish_after_days(fish_numbers, 80)}")
print(f"Part 2. After 256 days: {fish_after_days(fish_numbers, 256)}")