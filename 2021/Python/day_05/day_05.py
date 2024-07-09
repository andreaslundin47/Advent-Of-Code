from collections import defaultdict
from typing import NamedTuple

class Point(NamedTuple):
    x: int
    y: int

with open('input', 'r') as f :
    rows = [s.strip() for s in f.readlines()]
    segments = []
    for s in rows :
        start, end = s.split(' -> ')
        sx, sy = (int(v) for v in start.split(','))
        ex, ey = (int(v) for v in end.split(','))
        segments.append( ( Point(sx,sy), Point(ex,ey) ) )


def add_lines(lines) :
    sign = lambda x : -1 if x < 0 else (1 if x > 0 else 0)

    record = defaultdict(lambda: 0)
    for start, end in lines :
        x0, y0 = start; x1, y1 = end
        dx, dy = sign(x1-x0), sign(y1-y0)
        x, y = x0, y0

        while not (x == x1 and y == y1) :
            record[(x,y)] += 1
            x, y = x+dx, y+dy
        record[(x,y)] += 1
    return record


#### Part 1
straight = [s for s in segments if s[0].x == s[1].x or s[0].y == s[1].y]
taken = add_lines(straight)
overlaps = len([times for times in taken.values() if times > 1])
print(f"Part 1. Overlaps: {overlaps}")

#### Part 2
taken_2 = add_lines(segments)
overlaps = len([times for times in taken_2.values() if times > 1])
print(f"Part 2. Overlaps: {overlaps}")