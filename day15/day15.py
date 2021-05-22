data = [0, 1, 4, 13, 15, 12, 16]

def nth_number(seq, stop_time):
    mentions = {n:ts for ts,n in enumerate(data[:-1])}
    start_ts = len(data)
    last = data[-1]
    for ts in range(start_ts, stop_time):
        mentions[last], last = ts-1, (ts-1 - mentions.get(last, ts-1))
    return last

print(f"Part 1. 2020th number: {nth_number(data, 2020)}")
print(f"Part 2. 30'000'000th number: {nth_number(data, 30_000_000)}")